# levenshtein_rust

Levenshtein string distance algorithm in Rust.

## Installation

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add levenshtein_rust
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install levenshtein_rust
```

## Usage

```ruby
LevenshteinLite.distance("la", "là") # => 1
```

## Development

You'll need to have a recent [Rust](https://www.rust-lang.org/) compiler installed.

After checking out the repo, run `bin/setup` to install dependencies. Next, run `rake compile` to build the Rust code. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/undees/levenshtein_rust. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/undees/levenshtein_rust/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the LevenshteinRust project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/undees/levenshtein_rust/blob/main/CODE_OF_CONDUCT.md).
