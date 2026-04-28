
from transformers import TrainingArguments


def load_training_args():
    lr = 2e-4
    batch_size = 8
    num_epochs = 2

    try:
        training_args = TrainingArguments(
            output_dir = "bert-phishing-classifier-teacher",
            learning_rate = lr,
            per_device_train_batch_size = batch_size,
            per_device_eval_batch_size = batch_size,
            num_train_epochs = num_epochs,
            logging_strategy = "epoch",
            eval_strategy = "epoch",
            save_strategy = "epoch",
            load_best_model_at_end = True
        )
        return (lr, batch_size, num_epochs, training_args)

    except Exception as e:
        raise ValueError(f"Error while loading training arguments: {str(e)}")
    