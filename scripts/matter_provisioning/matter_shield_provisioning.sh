#!/bin/bash
source config.sh
source /etc/environment


echo "Perform Matter DAC Provisioning"
./matter_dac_provisioning.sh
echo "Print Shield Sticker"
python3 ./print_sticker.py
## update Security Monitor like
# ./bin/trustm_update_with_PBS_Auto -w 0xe0c9 -P "286F851DE7102015A9CC2EA713C78BA39B5A991CB74BEAB074F4E0B0440DD607AFA0DEE7D2D2647730D984328A0052842A672A520A734BDB54B9EE71E80EA4EC" -A "B8294A033A146C7470355E57780B0FAB413DE30B467F91AC6A80192C995C5DF0CD615DC77A379D9B7B7ADA58015D5364EAAADC56C2F5597E308A75E73B08B0D5" -I 0000050100000000
## but with scripted secret inputs