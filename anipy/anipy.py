from concurrent import futures
import logging

import grpc
import animalai_pb2
import animalai_pb2_grpc
import numpy as np

import torch
import torch.nn as nn
import torch.optim as optim
import torch.nn.functional as F

device = torch.device("cpu")

# DQN

k = 192
# fcl_units = 256
fcl_units = 4608 * 4
class DQN(nn.Module):

    def __init__(self):
        super(DQN, self).__init__()
        self.conv1 = nn.Conv2d(9, k, kernel_size=3, padding=1)
        self.bn1 = nn.BatchNorm2d(k)
        self.conv2 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn2 = nn.BatchNorm2d(k)
        self.conv3 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn3 = nn.BatchNorm2d(k)
        self.conv4 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn4 = nn.BatchNorm2d(k)
        self.conv5 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn5 = nn.BatchNorm2d(k)
        self.conv6 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn6 = nn.BatchNorm2d(k)
        self.conv7 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn7 = nn.BatchNorm2d(k)
        self.conv8 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn8 = nn.BatchNorm2d(k)
        self.conv9 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn9 = nn.BatchNorm2d(k)
        self.conv10 = nn.Conv2d(k, k, kernel_size=3, padding=1)
        self.bn10 = nn.BatchNorm2d(k)
        self.fcl1 = nn.Linear(k * 15, fcl_units)
        self.fcl2 = nn.Linear(fcl_units, 4608)

    def forward(self, x):
        x = F.relu(self.bn1(self.conv1(x)))
        x = F.relu(self.bn2(self.conv2(x)))
        x = F.relu(self.bn3(self.conv3(x)))
        x = F.relu(self.bn4(self.conv4(x)))
        x = F.relu(self.bn5(self.conv5(x)))
        x = F.relu(self.bn6(self.conv6(x)))
        x = F.relu(self.bn7(self.conv7(x)))
        x = F.relu(self.bn8(self.conv8(x)))
        x = F.relu(self.bn9(self.conv9(x)))
        x = F.relu(self.bn10(self.conv10(x)))
        x = F.relu(self.fcl1(x.view(-1, k * 15)))
        x = self.fcl2(x)
        return x.tanh()

class GreedyPlayer:
    def __init__(self, device):
        self.device = device
        self.model = DQN().to(device)
        checkpoint = torch.load("./model/model100000.pt", map_location=device)
        self.model.load_state_dict(checkpoint['state_dict'])
        self.model.eval()
        self.features = np.empty((1, 9, 5, 3), np.float32)

    def action(self, board, legal_moves):
        with torch.no_grad():
            self.features[0] = board
            state = torch.from_numpy(self.features).to(self.device)
            q = self.model(state)
            # 合法手に絞る
            next_actions = torch.tensor([legal_moves],
                                        device=self.device,
                                        dtype=torch.long)
            legal_q = q.gather(1, next_actions)
            return legal_moves[legal_q.argmax(dim=1).item()]


class AiPlayer(animalai_pb2_grpc.AiPlayerServicer):
    def __init__(self):
        self.player = GreedyPlayer(device)

    def ThinkAction(self, request, context):
        board = np.reshape(np.array(request.cells), (9, 5, 3))
        ret = self.player.action(board, request.legal_moves)
        return animalai_pb2.ActionRep(action=ret)

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    animalai_pb2_grpc.add_AiPlayerServicer_to_server(AiPlayer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    serve()