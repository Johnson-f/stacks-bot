#!/bin/bash

set -e

echo "🚀 Release Script"
echo "================="

# Check if there are changes to commit
if [[ -n $(git status -s) ]]; then
    echo ""
    read -p "📝 Enter commit message: " commit_msg
    
    if [[ -z "$commit_msg" ]]; then
        echo "❌ Commit message cannot be empty"
        exit 1
    fi
    
    echo "📦 Adding changes..."
    git add .
    
    echo "💾 Committing..."
    git commit -m "$commit_msg"
    
    echo "⬆️  Pushing to GitHub..."
    git push
    
    echo "✅ Changes committed and pushed"
else
    echo "✅ No changes to commit"
fi

# Create and push tag
echo ""
read -p "🏷️  Enter version number (e.g., 1.0.0): " version

if [[ -z "$version" ]]; then
    echo "❌ Version number cannot be empty"
    exit 1
fi

# Add 'v' prefix if not present
if [[ ! "$version" =~ ^v ]]; then
    version="v$version"
fi

echo "🏷️  Creating tag $version..."
git tag "$version"

echo "⬆️  Pushing tag to GitHub..."
git push origin "$version"

echo ""
echo "✅ Release $version created successfully!"
echo "🐳 Docker image will be built and pushed to Docker Hub automatically"
