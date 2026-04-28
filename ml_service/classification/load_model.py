from transformers import pipeline

def load_classification_model(model_name="prompt_injection_deberta", tokenizer_name="prompt_injection_deberta"):
    try:
        print("loading classification model")
        return pipeline(
            task="text-classification",
            model=f"../model/{model_name}",
            tokenizer=f"../model/{tokenizer_name}"
        )
    except Exception as e:
        raise RuntimeError(f"Error while loading the classification model: {str(e)}")






