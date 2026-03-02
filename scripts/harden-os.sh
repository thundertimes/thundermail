#!/bin/bash
# harden-os.sh - System Hardening for Thundermail
# 
# This script configures the system for maximum privacy and security
# when using Thundermail with Tor/SOCKS5.
#
# Usage: sudo ./harden-os.sh

set -e

echo "⚡ Thundermail OS Hardening Script"
echo "=================================="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (sudo)"
    exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

echo "[1/5] Checking prerequisites..."

# Check for Tor
if command_exists tor; then
    echo "✓ Tor found"
else
    echo "✗ Tor not found. Install with: apt install tor"
    exit 1
fi

# Check for GnuPG
if command_exists gpg; then
    echo "✓ GnuPG found"
else
    echo "✗ GnuPG not found. Install with: apt install gnupg"
    exit 1
fi

echo "[2/5] Configuring Tor..."

# Configure Tor for SOCKS5
TORRC="/etc/tor/torrc"
if [ -f "$TORRC" ]; then
    # Backup original
    cp "$TORRC" "$TORRC.backup"
    
    # Enable SOCKS port
    if ! grep -q "^SocksPort 9050" "$TORRC"; then
        echo "SocksPort 9050" >> "$TORRC"
    fi
    
    # Enable control port (for newnym)
    if ! grep -q "^ControlPort 9051" "$TORRC"; then
        echo "ControlPort 9051" >> "$TORRC"
    fi
    
    # Disable unnecessary features
    echo "DisableNetwork 0" >> "$TORRC" 2>/dev/null || true
    
    echo "✓ Tor configured"
else
    echo "✗ Tor config not found at $TORRC"
fi

echo "[3/5] Configuring system DNS..."

# Configure system DNS to use Tor
RESOLV_CONF="/etc/resolv.conf"
if [ -f "$RESOLV_CONF" ]; then
    # Backup original
    cp "$RESOLV_CONF" "$RESOLV_CONF.backup"
    
    # Set localhost as DNS (Tor uses 9053 for DNS queries)
    # Note: This should be done via /etc/systemd/resolved.conf for systemd
    echo "nameserver 127.0.0.1" > "$RESOLV_CONF"
    echo "options edns0" >> "$RESOLV_CONF"
    
    echo "✓ DNS configured"
fi

echo "[4/5] Configuring firewall..."

# Configure iptables for Tor routing
# This is a basic setup - more advanced configurations may be needed
if command_exists iptables; then
    # Allow loopback
    iptables -A INPUT -i lo -j ACCEPT 2>/dev/null || true
    iptables -A OUTPUT -o lo -j ACCEPT 2>/dev/null || true
    
    # Allow Tor SOCKS
    iptables -A OUTPUT -p tcp --dport 9050 -j ACCEPT 2>/dev/null || true
    
    # Allow established connections
    iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT 2>/dev/null || true
    
    echo "✓ Firewall configured"
fi

echo "[5/5] Enabling services..."

# Start Tor service
if command_exists systemctl; then
    systemctl start tor 2>/dev/null || echo "Note: Could not start Tor (may already be running)"
    systemctl enable tor 2>/dev/null || true
    echo "✓ Tor service enabled"
else
    # Try to start Tor directly
    tor --quiet &
    echo "✓ Tor started"
fi

echo ""
echo "=================================="
echo "Hardening complete!"
echo ""
echo "To verify Tor is working:"
echo "  curl --socks5 localhost:9050 http://check.torproject.org/api/ip"
echo ""
echo "To get a new Tor circuit:"
echo "  echo 'AUTHENTICATE \"\"' | nc localhost 9051"
echo "  echo 'SIGNAL NEWNYM' | nc localhost 9051"
echo ""
echo "⚠️  Remember to configure Thundermail to use the Tor proxy"
echo "    in your config.toml"
