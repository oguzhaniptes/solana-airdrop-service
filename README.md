# Solana Airdrop Service

## Linux/macOS Setup

### Create a Systemd Service

1. Open a terminal and create a systemd service file:
   ```sh
   sudo nano /etc/systemd/system/solana-airdrop.service
   ```
2. Add the following configuration:

   ```ini
   [Unit]
   Description=Solana Airdrop Service
   After=network.target

   [Service]
   ExecStart=/usr/bin/env cargo run --release
   WorkingDirectory=/path/to/solana-airdrop-service
   Restart=always
   User=your-username

   [Install]
   WantedBy=multi-user.target
   ```

3. Reload the systemd daemon:
   ```sh
   sudo systemctl daemon-reload
   ```
4. Enable and start the service:
   ```sh
   sudo systemctl enable solana-airdrop
   sudo systemctl start solana-airdrop
   ```
5. Check logs:
   ```sh
   journalctl -u solana-airdrop -f
   ```
6. Stop the service if needed:
   ```sh
   sudo systemctl stop solana-airdrop
   ```

## Windows Setup

### Build the Executable

```sh
cargo build --release
```

### Add Service Using Windows Task Scheduler

1. Press `Windows + R`, type `taskschd.msc`, and press Enter.
2. Click **"Create Basic Task"**.
3. Set the name as **"Solana Airdrop Service"**.
4. In the "Trigger" section, choose **"Daily"**.
5. In the "Action" section, select **"Start a program"**.
6. Under "Program/script", browse and select the built `solana-airdrop-service.exe` file.
7. Click **"Finish"**.

### Configure Automatic Execution Every 8 Hours

1. Open **Task Scheduler**.
2. Double-click the created task.
3. Go to the **"Triggers"** tab and click **"Edit"**.
4. Check **"Repeat task every 8 hours"**.

## Logging

### Linux/macOS

To monitor logs:

```sh
tail -f /tmp/solana_airdrop.log
```

If using systemd:

```sh
journalctl -u solana-airdrop -f
```

### Windows

To view logs in Windows, open the log file:

```sh
notepad C:\tmp\solana_airdrop.log
```

If the `C:\tmp` folder does not exist, create it:

```sh
mkdir C:\tmp
```
