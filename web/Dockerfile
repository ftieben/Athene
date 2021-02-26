FROM hayd/alpine-deno:1.7.2
EXPOSE 1993
WORKDIR /app
USER deno
ADD . .
RUN deno cache main.ts
CMD ["run", "--allow-net", "main.ts"]