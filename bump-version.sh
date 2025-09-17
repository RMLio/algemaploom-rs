#!/usr/bin/env bash

# exit if command fails
set -e

if [ -z "$1" ]
then
	echo 'Version parameter not given. Invoke as e.g. ./bump-version.sh 1.0.0'.
	exit 1
fi

# function to read `y` (yes) or `n` (no).
function yes_or_no {
	while true; do
		read -p "$* [y/n]: " yn
		case $yn in
			[Yy]*) return 1  ;;
			[Nn]*) echo "Aborted" ; return 0 ;;
		esac
	done
}

SCRIPTDIR=$(dirname $(readlink -f $0))

VERSION=$1
echo "Changing version to $VERSION"

echo 'Updating Cargo.toml...'
sed -i -e "s|^version = \".*\"|version = \"$VERSION\"|" Cargo.toml

echo 'Updating pom.xml...'
cd src/java/algemaploom/
mvn versions:set -DnewVersion=$VERSION
cd $SCRIPTDIR

echo 'Updating package.json...'
sed -i -e "s|\"version\": \".*\"|\"version\": \"$VERSION\"|" package.json

# There is no CHANGELOG yet!!
#if [ ! "$(yes_or_no 'Do you also want to add the version to CHANGELOG.md?')" ]
#then
#	changefrog -n $VERSION
#fi

tagname="v$VERSION"
if [ ! "$(yes_or_no 'Do you also want to commit the changes, create a git tag $tagname and push it?')" ]
then
	git add Cargo.toml package.json src/java/algemaploom/pom.xml
	git commit -m "Update version to $VERSION"
	git push origin
	git tag $tagname
	git push origin $tagname
fi

