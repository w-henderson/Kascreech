#!/usr/bin/bash

# KASCREECH DEPLOY SCRIPT
# RUNS IN WSL ONLY

vps_account=william@kascreech.ga
ssh_key=C:/Users/willi/.ssh/gcp-key
cargo_path=/home/william/.cargo/bin/cargo

echo "Building React front-end..."
cd client
cmd.exe /c npm run build
echo "Zipping build..."
zip -r -q static.zip build
cd ..

echo "Building Rust back-end..."
cd server
$cargo_path build --release
cd ..

echo "Uploading files..."
client_file_addr=`curl -s --upload-file client/static.zip https://transfer.sh/static.zip`
server_file_addr=`curl -s --upload-file server/target/release/kascreech https://transfer.sh/kascreech`

echo "Connecting to remote machine to finish deployment..."
cmd.exe /c ssh -i $ssh_key $vps_account "echo \"Stopping server...\"; sudo kill -s 9 \`pidof humphrey\`; sudo kill -s 9 \`pidof kascreech\`; echo \"Removing old version...\"; rm -rf static; rm -f kascreech; echo \"Installing new server...\"; curl -s -o kascreech $server_file_addr; sudo chmod +x kascreech; echo \"Installing client files...\"; curl -s -o static.zip $client_file_addr; unzip -q static.zip; mv build static; rm static.zip; echo \"Starting server again...\"; sudo ./start; echo \"Done!\"; exit;"

echo "Deployment successful!"