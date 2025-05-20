# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("levenshtein_rust.gemspec")

RbSys::ExtensionTask.new("levenshtein_rust", GEMSPEC) do |ext|
  ext.lib_dir = "lib/levenshtein_rust"
end

task default: %i[compile spec]
