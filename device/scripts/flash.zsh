PROGRAM="$1"

if [ -z "$PROGRAM" ]; then
  echo "ERROR: Missing program value"
  echo "$ picoflash [PROGRAM]"
  exit 1
fi

sudo openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" -c "program $PROGRAM verify reset exit"
