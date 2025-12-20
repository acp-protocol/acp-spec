#!/bin/bash

# Test Discord Webhook Script
# Usage: ./scripts/test-discord-webhook.sh <webhook-url>

set -e

WEBHOOK_URL="${1:-${DISCORD_WEBHOOK_URL}}"

if [ -z "$WEBHOOK_URL" ]; then
    echo "Error: Webhook URL required"
    echo "Usage: $0 <webhook-url>"
    echo "   or: DISCORD_WEBHOOK_URL=<url> $0"
    exit 1
fi

echo "Testing Discord webhook..."
echo "Webhook URL: ${WEBHOOK_URL:0:50}..."

# Test message payload
PAYLOAD=$(cat <<EOF
{
  "username": "ACP Bot",
  "avatar_url": "https://github.com/acp-protocol.png",
  "embeds": [{
    "title": "🧪 Discord Webhook Test",
    "description": "This is a test message to verify the Discord webhook is working correctly!\n\n**Test Details:**\n• Time: $(date -u +"%Y-%m-%d %H:%M:%S UTC")\n• Script: test-discord-webhook.sh\n\nIf you see this message, your webhook is configured correctly! ✅",
    "color": 5067324,
    "footer": {
      "text": "ACP Discord Integration Test"
    },
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  }]
}
EOF
)

# Send test message
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$WEBHOOK_URL" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD")

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | head -n-1)

if [ "$HTTP_CODE" -eq 204 ]; then
    echo "✅ Success! Webhook test message sent to Discord."
    echo "Check your #general channel to see the test message."
elif [ "$HTTP_CODE" -eq 404 ]; then
    echo "❌ Error: Webhook not found (404)"
    echo "Please check that your webhook URL is correct."
    exit 1
elif [ "$HTTP_CODE" -eq 401 ]; then
    echo "❌ Error: Unauthorized (401)"
    echo "Please check that your webhook URL is valid and not expired."
    exit 1
else
    echo "❌ Error: HTTP $HTTP_CODE"
    echo "Response: $BODY"
    exit 1
fi

