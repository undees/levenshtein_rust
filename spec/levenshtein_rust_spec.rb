# frozen_string_literal: true

RSpec.describe LevenshteinRust do
  it "has a version number" do
    expect(LevenshteinRust::VERSION).not_to be nil
  end

  it "is callable from Ruby" do
    expect(LevenshteinRust.hello("Ruby"))
      .to eq("Hello from Rust, Ruby!")
  end
end
