import torch
import torch.optim as optim
from torch.utils.data import Dataset

class OnChainDataset(Dataset):
    def __init__(self, data_stream):
        self.temporal_features = data_stream['sequence_data']
        self.spatial_features = data_stream['wallet_graph']
        
    def __len__(self):
        return len(self.temporal_features)
    
    def __getitem__(self, idx):
        return {
            'temporal': torch.tensor(self.temporal_features[idx], 
            'spatial': torch.tensor(self.spatial_features[idx])
        }

class FederatedTrainer:
    def __init__(self, model, dp_epsilon=1.0):
        self.model = model
        self.dp_epsilon = dp_epsilon
        
    def add_dp_noise(self, gradients):
        sigma = (1.0 / self.dp_epsilon) * torch.sqrt(torch.tensor(2.0 * torch.log(torch.tensor(1.25/0.01)))
        for grad in gradients:
            grad += torch.normal(0, sigma, grad.shape)
        return gradients
    
    def client_update(self, data_loader, epochs=1):
        self.model.train()
        optimizer = optim.SGD(self.model.parameters(), lr=0.01)
        
        for _ in range(epochs):
            for batch in data_loader:
                outputs = self.model(batch['temporal'])
                loss = nn.BCELoss()(outputs, batch['spatial'])
                loss.backward()
                optimizer.step()
                optimizer.zero_grad()
                
        return [param.grad.clone() for param in self.model.parameters()]
