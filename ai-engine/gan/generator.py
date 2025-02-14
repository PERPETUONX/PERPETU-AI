import torch
import torch.nn as nn

class SatireGenerator(nn.Module):
    """Generates satirical memes from transaction data"""
    def __init__(self, latent_dim=256):
        super().__init__()
        self.text_encoder = nn.TransformerEncoder(
            nn.TransformerEncoderLayer(d_model=latent_dim, nhead=8),
            num_layers=3
        )
        self.image_decoder = nn.Sequential(
            nn.ConvTranspose2d(latent_dim, 128, 4),
            nn.BatchNorm2d(128),
            nn.GELU(),
            nn.ConvTranspose2d(128, 64, 4),
            nn.Tanh()
        )

    def forward(self, tx_features):
        encoded = self.text_encoder(tx_features)
        return self.image_decoder(encoded.view(-1, 256, 1, 1))

class ArtCritic(nn.Module):
    """Evaluates meme painfulness score"""
    def __init__(self):
        super().__init__()
        self.discriminator = nn.Sequential(
            nn.Conv2d(3, 64, 4, 2),
            nn.LeakyReLU(0.2),
            nn.Conv2d(64, 128, 4, 2),
            nn.InstanceNorm2d(128),
            nn.LeakyReLU(0.2),
            nn.Flatten(),
            nn.Linear(128*13*13, 1),
            nn.Sigmoid()
        )

    def forward(self, images):
        return self.discriminator(images)
