FROM rust:latest
WORKDIR /code
COPY fibonacci/ /code/
RUN cargo build --release

FROM ruby:3.0.2
WORKDIR /code
COPY ruby/Gemfile ruby/Gemfile.lock /code/
RUN bundle install
COPY --from=0 /code/target/release /fibonacci
COPY ruby/ /code/
CMD ["bundle", "exec", "ruby", "main.rb"]
