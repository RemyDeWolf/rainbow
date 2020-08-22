
FROM gradle:jdk11 AS build

#COPY --chown=gradle:gradle . /home/gradle/src
ADD --chown=gradle:gradle build.gradle /home/gradle/src/build.gradle
ADD src/main/java/compute/Main.java /home/gradle/src/src/main/java/compute/Main.java

# copy the script to compute
ARG COMPUTE
ADD src/main/java/compute/$COMPUTE.java /home/gradle/src/src/main/java/compute/Compute.java

WORKDIR /home/gradle/src
RUN gradle build --no-daemon 

# runtime image (jre only)
FROM alpine:latest

RUN apk --update add openjdk11-jre

RUN mkdir /app
RUN mkdir /data
ARG COMPUTE
ENV COMPUTE=$COMPUTE
COPY --from=build /home/gradle/src/build/libs/*.jar /app/compute.jar
ENTRYPOINT ["java", "-jar", "/app/compute.jar"]
