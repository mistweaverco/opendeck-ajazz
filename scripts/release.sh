#!/usr/bin/env bash

if [ -z "$VERSION" ]; then echo "Error: VERSION is not set"; exit 1; fi
if [ -z "$PLATFORM" ]; then echo "Error: PLATFORM is not set"; exit 1; fi

BIN_NAME="opendeck-ajazz"
RELEASE_ACTION="create"
GH_TAG="v$VERSION"
FILES=()

LINUX_FILES=(
  "src-tauri/target/release/bundle/deb/${BIN_NAME}_${VERSION}_amd64.deb"
  "src-tauri/target/release/bundle/rpm/${BIN_NAME}-${VERSION}-1.x86_64.rpm"
  "src-tauri/target/release/bundle/appimage/${BIN_NAME}_${VERSION}_amd64.AppImage"
)

# Strip version from filename
# e.g.
# - opendeck-ajazz_1.0.0_amd64.deb -> opendeck-ajazz.amd64.deb
# - opendeck-ajazz_1.0.0-1.x86_64.rpm -> opendeck-ajazz.x86_64.rpm
# - opendeck-ajazz_1.0.0_amd64.AppImage -> opendeck-ajazz.AppImage
strip_version_from_filename() {
  local filename="$1"
  local base_filename
  base_filename=$(basename "$filename")
  local stripped_filename
  stripped_filename=$(echo "$base_filename" | sed -E "s/${BIN_NAME}(\-|_)[0-9]+\.[0-9]+\.[0-9]+(\-[0-9]+)?(\.|_)(.+)/${BIN_NAME}.\4/")
  echo "$stripped_filename"

}

set_release_action() {
  if gh release view "$GH_TAG" --json id --jq .id > /dev/null 2>&1; then
    echo "Release $GH_TAG already exists, updating it"
    RELEASE_ACTION="edit"
  else
    echo "Release $GH_TAG does not exist, creating it"
    RELEASE_ACTION="create"
  fi
}

pad_lines() {
  local text="$1"
  local line
  local lines
  local output=""
  IFS=$'\n' read -rd '' -a lines <<<"$text"
  for line in "${lines[@]}"; do
    output+=$'\t'"- ${line}"$'\n'
  done
  echo "$output"
}

check_files_exist() {
  files=()
  for file in "${FILES[@]}"; do
    if [ ! -f "$file" ]; then
      files+=("$file")
    fi
  done
  if [ ${#files[@]} -gt 0 ]; then
    echo "Error: the following files do not exist:"
    for file in "${files[@]}"; do
      printf " - %s\n" "$file"
    echo -ne "\tThis is the content of the dist directory:\n"
    local dir_contents
    dir_contents=$(ls -1 "$(dirname "${file}")")
    pad_lines "$dir_contents"
    done
    exit 1
  fi
}

clone_file_new_name() {
  local file="$1"
  local new_name
  new_name="$(dirname "$file")/$(strip_version_from_filename "$file")"
  cp "$file" "$new_name"
  echo "$new_name"
}

set_files_based_on_platform() {
  case $PLATFORM in
    linux)
      for file in "${LINUX_FILES[@]}"; do
        new_name=$(clone_file_new_name "$file")
        FILES+=("$new_name")
      done
      ;;
    *)
      echo "Error: PLATFORM $PLATFORM is not supported"
      exit 1
      ;;
  esac
}

do_gh_release() {
  if [ "$RELEASE_ACTION" == "edit" ]; then
    echo "Overwriting existing release $GH_TAG"
    gh release upload --clobber "$GH_TAG" "${FILES[@]}"
  else
    echo "Creating new release $GH_TAG"
    gh release create --generate-notes "$GH_TAG" "${FILES[@]}" || RELEASE_ACTION="edit" && do_gh_release
  fi
}

release() {
  set_release_action
  set_files_based_on_platform
  check_files_exist
  do_gh_release
}

release
