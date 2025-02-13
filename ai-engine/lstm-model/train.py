import torch
import torch.nn as nn

class ChainLSTM(nn.Module):
    """On-chain compatible LSTM with quantization"""
    def __init__(self, input_size=12, hidden_size=64):
        super().__init__()
        self.lstm = nn.LSTM(input_size, hidden_size, batch_first=True)
        self.quant = torch.quantization.QuantStub()
        self.dequant = torch.quantization.DeQuantStub()

    def forward(self, x):
        x = self.quant(x)
        out, _ = self.lstm(x)
        out = self.dequant(out[:, -1, :])
        return torch.sigmoid(out)

def federated_update(global_model, client_updates):
    """Federated averaging with differential privacy"""
    with torch.no_grad():
        for param in global_model.parameters():
            param.zero_()
        for update in client_updates:
            for g_param, c_param in zip(global_model.parameters(), update):
                noise = torch.randn_like(g_param) * 0.01  # DP noise
                g_param += (c_param + noise) / len(client_updates)
    return global_model
