echo "Starting builds ..."
cargo build --release

# echo "Moving new executable to jkctools ..."
# cp ".\target\release\opinion-renamer.exe" "C:\jkctools\opinion-renamer\reop.exe"
# cp ".\target\release\opinion-renamer.exe" "C:\jkctools\opinion-renamer\opinion-renamer.exe"
# cp ".\pdfium.dll" "C:\jkctools\opinion-renamer\pdfium.dll"


DEST_DIRECTORY1="C:\Users\jason\OneDrive\Apps\opinion-renamer"
# DIRECTORY2="C:\mount\C&S Dropbox\jkctools\tscript"

# Create the Destination Directory if it doesn't exist
if [ ! -d "$DEST_DIRECTORY1" ]; then
  mkdir -p "$DEST_DIRECTORY1"
fi

if [ -d "$DEST_DIRECTORY1" ]; then
  echo "$DEST_DIRECTORY1 does exist."
  echo "Moving new executables to Dropbox jkctools ..."
  cp ".\target\release\opinion-renamer.exe" "$DEST_DIRECTORY1\reop.exe"
  cp ".\target\release\opinion-renamer.exe" "$DEST_DIRECTORY1\opinion-renamer.exe"
  cp ".\pdfium.dll" "$DEST_DIRECTORY1\pdfium.dll"

  echo "Finished moving new executables to $DEST_DIRECTORY1."
fi



# if [ -d "$DIRECTORY2" ]; then
#   echo "$DIRECTORY2 does exist."
#   echo "Moving new executables to Dropbox jkctools ..."
#   cp ".\target\release\opinion-renamer.exe" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\reop.exe"
#   cp ".\target\release\opinion-renamer.exe" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\opinion-renamer.exe"
#   cp ".\pdfium.dll" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\pdfium.dll"
# fi