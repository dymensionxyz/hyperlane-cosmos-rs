#!/bin/bash
set -e

echo "🔍 Verifying protobuf definitions are up to date..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if a field exists in the Rust generated code
check_field_exists() {
    local file=$1
    local struct_name=$2
    local field_name=$3
    
    if grep -q "pub $field_name:" "$file"; then
        echo -e "${GREEN}✓${NC} Field '$field_name' found in $struct_name"
        return 0
    else
        echo -e "${RED}✗${NC} Field '$field_name' NOT found in $struct_name"
        return 1
    fi
}

# Check HlMetadata struct
FORWARD_RS="src/prost/dymensionxyz.dymension.forward.rs"

if [ ! -f "$FORWARD_RS" ]; then
    echo -e "${RED}Error: $FORWARD_RS not found!${NC}"
    exit 1
fi

echo -e "\n${YELLOW}Checking HlMetadata struct...${NC}"

# Check all three fields exist
ERRORS=0

check_field_exists "$FORWARD_RS" "HlMetadata" "hook_forward_to_ibc" || ((ERRORS++))
check_field_exists "$FORWARD_RS" "HlMetadata" "kaspa" || ((ERRORS++))
check_field_exists "$FORWARD_RS" "HlMetadata" "hook_forward_to_hl" || ((ERRORS++))

# Check the field tags are correct
echo -e "\n${YELLOW}Checking field tags...${NC}"

if grep -q 'tag="1".*hook_forward_to_ibc' "$FORWARD_RS"; then
    echo -e "${GREEN}✓${NC} hook_forward_to_ibc has correct tag (1)"
else
    echo -e "${RED}✗${NC} hook_forward_to_ibc has incorrect tag"
    ((ERRORS++))
fi

if grep -q 'tag="2".*kaspa' "$FORWARD_RS"; then
    echo -e "${GREEN}✓${NC} kaspa has correct tag (2)"
else
    echo -e "${RED}✗${NC} kaspa has incorrect tag"
    ((ERRORS++))
fi

if grep -q 'tag="3".*hook_forward_to_hl' "$FORWARD_RS"; then
    echo -e "${GREEN}✓${NC} hook_forward_to_hl has correct tag (3)"
else
    echo -e "${RED}✗${NC} hook_forward_to_hl has incorrect tag"
    ((ERRORS++))
fi

# Summary
echo -e "\n${YELLOW}Summary:${NC}"
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✅ All protobuf checks passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Found $ERRORS errors in protobuf definitions${NC}"
    echo -e "${YELLOW}Run 'make proto-gen' to regenerate the protobuf files${NC}"
    exit 1
fi