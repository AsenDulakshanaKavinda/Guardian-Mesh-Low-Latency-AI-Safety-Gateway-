from utils import get_logger, config

log = get_logger(__file__)

def main():
    log.info("Hello from ml-service!")
    print(config)

if __name__ == "__main__":
    main()
