
from transformers import AutoTokenizer, AutoModelForSequenceClassification
from transformers import pipeline


class TrainingModel:

    def __init__(self, model_name: str, model_save_path: str, num_labels: int, id2label: dict, label2id: dict):
        self.model_name = model_name
        self.model_path = model_save_path

        self.id2label: dict = id2label
        self.label2id: dict = label2id
        self.num_labels: int = num_labels

        self.model = None
        self.tokenizer = None


    # load the base model and tokenizer from the specified path
    def load_base_model(self) -> None:
        if self.model is None:
            self.model = AutoModelForSequenceClassification.from_pretrained(
                model_name = self.model_name,
                num_labels = self.num_labels,
                id2label = self.id2label,
                label2id = self.label2id,
            )
        
        if self.tokenizer is None:
            self.tokenizer = AutoTokenizer.from_pretrained(
                model_name = self.model_name
            )


    # set trainable parameters
    def set_trainable_parameters(self, params: list) -> None:
        for name, param in self.model.named_parameters():
            if name in params:
                param.requires_grad = True
            else:
                param.requires_grad = False


    def get_trainable_model(self) -> AutoModelForSequenceClassification:
        if self.model is None:
            raise ValueError("Model not loaded. Please call load_base_model() first.")
        return self.model


    def get_tokenizer(self) -> AutoTokenizer:
        if self.tokenizer is None:
            raise ValueError("Tokenizer not loaded. Please call load_base_model() first.")
        return self.tokenizer
    
    

        