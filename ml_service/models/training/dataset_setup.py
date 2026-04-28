from datasets import load_dataset
from transformers import AutoTokenizer

class TrainingDataset:

    def __init__(self, tokenizer: AutoTokenizer):
        self.dataset_name = None
        self.dataset_save_path = None
        self.dataset_dict = None
        self.dataset_version = None
        self.tokenizer = tokenizer

    def load_raw_dataset(self) -> None:
        try:
            self.dataset_dict = load_dataset(
                self.dataset_name,
                self.dataset_version 
            )
        except Exception as e:
            raise RuntimeError(f"Error while saving the dataset: {str(e)}")
    

    def get_dataset(self, ):
        if not self.dataset_dict:
            raise ValueError("Dataset not loaded. Please call load_raw_dataset() first.")
        return self.dataset_dict
