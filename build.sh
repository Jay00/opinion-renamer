echo "Starting builds ..."
cargo build --release

echo "Moving new executable to jkctools ..."
cp ".\target\release\opinion-renamer.exe" "C:\jkctools\opinion-renamer\reop.exe"
cp ".\target\release\opinion-renamer.exe" "C:\jkctools\opinion-renamer\opinion-renamer.exe"
cp ".\pdfium.dll" "C:\jkctools\opinion-renamer\pdfium.dll"


DIRECTORY1="C:\Users\jason\C&S Dropbox\jkctools\tscript"
DIRECTORY2="C:\mount\C&S Dropbox\jkctools\tscript"

if [ -d "$DIRECTORY1" ]; then
  echo "$DIRECTORY1 does exist."
  echo "Moving new executables to Dropbox jkctools ..."
  cp ".\target\release\opinion-renamer.exe" "C:\Users\jason\C&S Dropbox\jkctools\opinion-renamer\reop.exe"
  cp ".\target\release\opinion-renamer.exe" "C:\Users\jason\C&S Dropbox\jkctools\opinion-renamer\opinion-renamer.exe"
  cp ".\pdfium.dll" "C:\Users\jason\C&S Dropbox\jkctools\opinion-renamer\pdfium.dll"
fi



if [ -d "$DIRECTORY2" ]; then
  echo "$DIRECTORY2 does exist."
  echo "Moving new executables to Dropbox jkctools ..."
  cp ".\target\release\opinion-renamer.exe" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\reop.exe"
  cp ".\target\release\opinion-renamer.exe" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\opinion-renamer.exe"
  cp ".\pdfium.dll" "C:\mount\C&S Dropbox\jkctools\opinion-renamer\pdfium.dll"
fi