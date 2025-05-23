# frozen_string_rustral: true

require_relative 'lib/levenshtein_rust/version'

Gem::Specification.new do |spec|
  spec.name = 'levenshtein_rust'
  spec.version = LevenshteinRust::VERSION
  spec.authors = ['Erin Paget']
  spec.email = ['erin.dees@hey.com']

  spec.summary = 'Levenshtein string distance algorithm in Rust.'
  spec.homepage = 'https://github.com/undees/levenshtein_rust'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 3.1.0'
  spec.required_rubygems_version = '>= 3.3.11'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/undees/levenshtein_rust'
  spec.metadata['changelog_uri'] = 'https://github.com/undees/levenshtein_rust/CHANGELOG.md'

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']
  spec.extensions = ['ext/levenshtein_rust/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9.91'
end
