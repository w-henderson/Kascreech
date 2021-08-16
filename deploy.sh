#!/usr/bin/bash

# KASCREECH DEPLOY SCRIPT
# RUNS IN WSL ONLY

vps_account=william@ssh.kascreech.ga
ssh_key=C:/Users/willi/.ssh/gcp-key
cargo_path=/home/william/.cargo/bin/cargo

echo "Building server..."
cd server
$cargo_path build --release
cd ..

echo "Uploading files..."
server_file_addr=`curl -s --upload-file server/target/release/kascreech https://transfer.sh/kascreech`

echo "Connecting to remote machine to finish deployment..."
cmd.exe /c ssh -i $ssh_key $vps_account "echo \"Stopping server...\"; sudo kill -s 9 \`pidof kascreech\`; echo \"Removing old version...\"; rm -f kascreech; echo \"Installing new server...\"; curl -s -o kascreech $server_file_addr; sudo chmod +x kascreech; echo \"Starting server again...\"; sudo ./start; echo \"Done!\"; exit;"

echo "Deployment successful!"