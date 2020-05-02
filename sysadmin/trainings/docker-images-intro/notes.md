## Notes

Each docker container is actually a Linux process. More on this in the docker
container class.

`docker run` command is `create` + `start` all in one
 * `-d` - detached, or daemonized
 * `--rm` - remove container after it is finished running (not by default)
 * `-p` - port forwarding - more in container

Each container has its own network stack. We can map a port from host to the
container.

**Dockerfile**

 * `FROM` - base image to start from; can be 'scratch' (unusual)  
   *alpine* is a common base image - small, offers a shell
 * `RUN` - executes a command
 * `ADD` - copy files into the image (very similar to `COPY`)
 * `ENV` - set an environment variable with a default variable
 * `EXPOSE` - tells docker to expose by default a port (that the app is using)
 * `CMD` - not the same thing as `RUN` - that is changing the image; this  
   is setting some metadata to tell what will be started when the container  
   will be started

Note that everything in the current directory is uploaded to the docker daemon.
This can be adjusted with `.dockerignore` file (e.g.: to exclude the `.git`
directory of a git repo).

**Docker Hub**

Not mandatory. Can also use a private registry in a company.

id: catalingheorghe

 * `docker login` - tells the docker daemon to login to a registry (dockerhub  
   by default)
 * `docker logout`

On Linux, `~/.docker/config.json` stores auth in base64 encoding.

Registry has an API that you can query. E.g.:

```
$ curl -s -S \
  "https://registry.hub.docker.com/v2/repositories/library/alpine/tags/" \
  | jq '."results"[]["name"]' | sort
```

Note: also the docker daemon has an API that can be used. You can do anything
that you can do from any client. Good for CI/CD pipelines.

**Building an image**

 * `docker build -t ... .`
    * `-t` - tag image as well - dockerhubid/reponame:latest
    * `.` - this is important, it sets the build context
 * `docker push dockerhubid/reponame:latest`
    * pushes all the filesystem layers of the image upstream
    * now it can be pulled by you or others
 * `docker pull dockerhubid/reponame:tag`
 * `docker search node` / `docker search spkane`

Running the container - same as the initial docker run example.

The **build process** is

 - start from an image
 - create an intermediate container from that image
 - run the command
 - create a snapshot of that container
 - remove intermediate container

Note that adding files doesn't require the intermediate container because you
already know what files will be changed, aka added. So, just a new snapshot is 
created.

Note also that metadata commands, like `ENV`, `EXPORT`, `CMD`, required an
intermediate container even though they don't modify files in the image. They
to modify some metadata somewhere.

**How small can an image be?**

 * `docker export container-name -o export.tar` - exports a container

Example: spkane/outyet:1.9.4-small

Containers are isolated, network and filesystem. So a minimal container must
have the basic files for Linux userspace to work (some dev, etc and virtual
file systems). dev files: console, pts folder, shm folder.

The bare minimum in addition is a statically compiled binary and its required
data (like certificates).

**Debug building images**

 * `docker run --rm -it spkane/outyet:1.9.4-small /bin/sh`
   * `-it` - interactive and attach current stdin and out
   * this will not work because there is no `/bin/sh` to start in that  
     docker image -> the image is overoptimized, there is no shell to debug

**-- Multistage builds**

Why? Separate build requirements from deployment requirements.

Multiple FROM statements in a Dockerfile. 
E.g.: one build image for development, where you have full go tools; another 
image for deployment, another FROM statement which can start from 'scratch` - 
in this case the `COPY` has a --from=<index of FROM> option that can take files
from the previous images

**Layering**

 * `docker tag imageid tagname`
 * `docker history image`
   * shows how a image was created, all of the images it is made up of,  
     including size

Each image layer is imutable. Each line of a docker file create an imutable 
layer of the image. If you delete files in a subsequent it does not remove the
files from previous layers - it only masks them.

**Timing - order matters**

The time of building an image is also important, in a CI/CD world.

 * `docker build --no-cache .` - from scratch (no-cache)

Layers that contain files that will not be touched during development, must be
first, to avoid rebuild things.

Don't put your "source code", that you will modify in the first layers. Each
change will make all the following layers rebuild. The build cache will not 
be put to use (there will be no "Using cache" anymore - because the hash of 
the layer won't match anymore).

google - "best practices docker images ruby/node/python etc"

One good general advice is to put the vendor dependecies in your repositories.

**Private registry**




## Other

other trainings:

 - introduction to containers
 - ci/cd with docker

git note:

`git clone .... --config core.autocrlf=input` - this ensures that hosts on 
windows don't push back code with windows line endings, basically that the
line endings remain the same in the repository


