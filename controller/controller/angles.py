from typing import List
import time
import socket


class Angles:
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    neutral_angles = [
        60.0, 60.0, 60.0,
        60.0, 60.0, 60.0,
        60.0, 60.0, 60.0,

        60.0, 60.0, 60.0,
        60.0, 60.0, 60.0,
        60.0, 60.0, 60.0,
    ]
    angles = neutral_angles.copy()

    def set_angles(self, angles: List[float]):
        self.angles = angles

        msg = ','.join(map(str, self.angles))

        print(msg)

        self.client_socket.sendto(str.encode(msg), ("127.0.0.1", 5546))

    def set_angle(self, index: int, value: float):
        new_angles = self.angles.copy()

        new_angles[index] = value

        self.set_angles(new_angles)

    def reset_angles(self):
        self.set_angles(self.neutral_angles)

# angles = Angles()

# angles.reset_angles()
# time.sleep(0.5)

# # while True:
# for i in [0, 3]:
#     legs = [0 + i, 6 + i, 12 + i]

#     for i in legs:
#         angles.set_angle(i, 50)
#         angles.set_angle(i + 1, 40)
#         angles.set_angle(i + 2, 80)
#     time.sleep(0.5)

#     # while True:
#     for i in legs:
#         angles.set_angle(i, 40)
#         angles.set_angle(i + 1, 60)
#         angles.set_angle(i + 2, 80)
#     time.sleep(0.5)

#     # while True:
#     for i in legs:
#         angles.set_angle(i, 60)
#         angles.set_angle(i + 1, 60)
#         angles.set_angle(i + 2, 60)
#     time.sleep(0.5)
