from utils import get_logger, config
from model import load_nlp_model

import grpc
from concurrent import futures
import ner_proto_pb2
import ner_proto_pb2_grpc


log = get_logger(__file__)


class NERServer(ner_proto_pb2_grpc.NERServiceServicer):
    # Initialize the NER model when the server starts
    def __init__(self):
        # Load the NER model once when the server starts to avoid reloading it for every request
        self.nlp = load_nlp_model("en_core_web_sm")

    def ExtractEntities(self, request, context):
        # Process the input text and extract entities using the loaded NER model
        prompt = self.nlp(request.text)
        entities = []

        for ent in prompt.ents:
            entity_msg = ner_proto_pb2.Entity(
                text=ent.text,
                label=ent.label_,
                start=ent.start_char,
                end=ent.end_char
            )
            entities.append(entity_msg)
        
        return ner_proto_pb2.NERResponse(entities=entities)


def server():
    # This function starts the gRPC server and listens for incoming requests.
    try :
        server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))

        ner_proto_pb2_grpc.add_NERServiceServicer_to_server(
            NERServer(), server
        )

        server.add_insecure_port('[::]:50051')
        server.start()

        log.info("Server started on port 50051")
        server.wait_for_termination()
        
    except Exception as e:
        log.error("Error while starting the server.")
        raise RuntimeError(f"Error while starting server: {str(e)}")


if __name__ == "__main__":
    server()

