
from transformers import DataCollatorWithPadding

from models.training_setup.model_setup import TrainingModel
from models.training_setup.dataset_setup import TrainingDataset
from models.training_setup.training_parm import load_training_args
from models.training_setup.trainer import trainer

# setup model for fine tuning
model = TrainingModel(
    model_name="",
    model_save_path="",
    num_labels=0,
    id2label={},
    label2id={}
)

model.load_base_model()
model.set_trainable_parameters(params=[])
trainable_model = model.get_trainable_model()
tokenizer = model.get_tokenizer()

# setup database for fine tuning
training_dataset = TrainingDataset(tokenizer=tokenizer)
training_dataset.load_raw_dataset()
dataset = training_dataset.get_dataset()

# data preprocessing
def preprocessing_function(examples):
  return tokenizer(examples["text"], truncation=True)
tokenized_data = dataset.map(preprocessing_function, batched=True)

data_collator = DataCollatorWithPadding(tokenizer=tokenizer)

training_args = load_training_args()

fine_tuner = trainer(model, training_args, tokenized_data, data_collator, compute_metrics="")

fine_tuner.train()







"""
1. model
2. dataset


"""


