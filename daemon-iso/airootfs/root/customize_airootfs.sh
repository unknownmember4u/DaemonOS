#!/usr/bin/env bash

# Create the live user daemonos
useradd -m -G wheel,video,audio,input -s /bin/bash daemonos

# Configure empty password (passwordless login/sudo)
passwd -d daemonos
