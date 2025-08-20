import time
import re
import hid


LIST_RE = re.compile(r"\[\s*(.*?)\s*\]")
BYTE_RE = re.compile(r"(0x[0-9A-Fa-f]+|\d+)")

class UfcWriter:
    def __init__(self, PID=None, VID=None, writeDelay=0.01):
        self.PID = PID
        self.VID = VID
        self.write_delay = writeDelay

        self.INIT_PATH = "ufcInit.txt"
        self.device = hid.device()
        self.device.open(self.VID, self.PID)
        count = self.send_init_from_file()

        print(f"Sent {count} init packets from {self.INIT_PATH}")

    def send_init_from_file(self) -> int:
        sent = 0
        prefix = "02d0be0000064c"
        prefix += "08";
        suffix = "0000"
        string = "1000 1111 1111 1111 1111 0000 0000 0000"
        string = string.strip()
        string = string.replace(" ", "")
        string = string[4:8] + string[0:4] + string[12:16] + string[8:12] + string[20:24] + string[16:20] + string[28:32] + string[24:28]
        string = f"{int(string, 2):0{len(string)//4}x}"
        prefix = prefix + string + suffix
        self.device.write(bytes.fromhex(prefix))

        # with open(self.INIT_PATH, "r", encoding="utf-8") as f:
        #     for raw in f:
        #         if not raw.startswith("#"):
        #             line = bytes.fromhex(raw.strip())
        #             print(line)
        #             self.device.write(line)
        #             sent += 1
        #             time.sleep(0.001)
        return sent


VID = 0x4098
PID = 0xBEDE
WRITE_DELAY_S = 0.01
ufcWriter = UfcWriter(PID, VID, WRITE_DELAY_S)
