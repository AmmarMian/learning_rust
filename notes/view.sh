# ========================================
# FileName: view.sh
# Date: 22 juin 2023 - 10:49
# Author: Ammar Mian
# Email: ammar.mian@univ-smb.fr
# GitHub: https://github.com/ammarmian
# Brief: View hugo website
# =========================================

BROWSER=firefox
URL=http://localhost:1313
hugo server > /dev/null 2>&1 &
SERVER_PID=$!
sleep 1

$BROWSER $URL &

read BLABLA -p "Press any key to stop server and exit"
kill $SERVER_PID
