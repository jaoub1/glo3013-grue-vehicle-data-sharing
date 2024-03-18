# This is the Continuous Deployment Docker container.
# Use the "Dockerfile_rust" file instead to compile and run the program.

FROM debian:12-slim

COPY ./deploy_dir /deploy_dir
WORKDIR /deploy_dir
EXPOSE 8081

CMD ["./executable.run", "--address", "0.0.0.0", "--port", "8081"]
