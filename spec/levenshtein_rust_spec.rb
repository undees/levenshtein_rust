# frozen_string_literal: true

RSpec::Matchers.define :measure_the_distance_between do |s1, s2|
  chain(:as) { |amount| @amount = amount }
  match { |klass| klass.distance(s1, s2) == @amount }

  failure_message do |klass|
    super() + ", but it was #{klass.distance(s1, s2)}"
  end
end

RSpec::Matchers.define :raise_argument_error_for do |*args|
  chain(:containing) { |containing| @containing = containing }

  match do |klass|
    klass.distance(*args)
    false
  rescue ArgumentError => e
    !@containing || e.message.include?(@containing)
  end

  failure_message do
    modifier = @containing ?
                 " containing #{@containing.inspect}" :
                 ""
    "#{super()}#{modifier}, but it didn't"
  end
end

RSpec::Matchers
  .alias_matcher :raise_argument_error_for_no_args,
                 :raise_argument_error_for

RSpec::Matchers.define :raise_type_error_for do |*args|
  match do |klass|
    klass.distance(*args)
    false
  rescue TypeError
    true
  end

  failure_message { super() + ", but it didn't" }
end

RSpec.describe LevenshteinRust do
  subject { LevenshteinRust }

  describe '::VERSION' do
    it 'represents the version number' do
      expect(LevenshteinRust::VERSION).not_to be nil
    end
  end

  describe '.distance' do
    it { should measure_the_distance_between('',  '' ).as(0) }
    it { should measure_the_distance_between('',  'a').as(1) }
    it { should measure_the_distance_between('a', '' ).as(1) }
    it { should measure_the_distance_between('a', 'b').as(1) }

    it { should measure_the_distance_between('abc', 'azc'   ).as(1) }
    it { should measure_the_distance_between('abc', 'acb'   ).as(2) }
    it { should measure_the_distance_between('abc', 'ac'    ).as(1) }
    it { should measure_the_distance_between('ac' , 'abc'   ).as(1) }

    it { should measure_the_distance_between('la', 'là').as(1) }

    it {
      should measure_the_distance_between(
        'correct horsE battery staple',
        'correct battEry horse staple'
      ).as(12)
    }

    it 'is symmetrical' do
      expect(subject.distance('kitten', 'sitting'))
        .to eq(subject.distance('sitting', 'kitten'))
    end

    it 'measures the distance between long mismatched strings' do
      expect(subject.distance('a' * 1000, 'b' * 1000)).to eq(1000)
    end

    it 'considers composed / decomposed Unicode codepoints as different' do
      e_followed_by_combining_accent = "\u0065\u0301"
      single_char_accented_e = "\u00e9"

      expect(subject.distance(
               e_followed_by_combining_accent,
               single_char_accented_e
             )).to eq(2)
    end

    context 'with the wrong number of arguments' do
      it { should raise_argument_error_for_no_args }
      it { should raise_argument_error_for(nil) }
      it { should raise_argument_error_for("only_one_arg") }
      it { should raise_argument_error_for("a", "b", "c") }
    end

    context 'with the wrong argument types' do
      it { should raise_type_error_for("only_one_valid_arg", 2) }
      it { should raise_type_error_for(2, "only_one_valid_arg") }
      it { should raise_type_error_for(nil, nil) }
      it { should raise_type_error_for(1, 2) }
    end

    context 'with different encodings' do
      it {
        should raise_argument_error_for(
          "ascii".encode(Encoding::ASCII_8BIT),
          "utf-8").containing("Expected UTF-8 encoding")
      }

      it {
        should raise_argument_error_for(
          "bad-utf-u8\xff".dup.force_encoding(Encoding::UTF_8),
          "utf-8").containing("Invalid UTF-8")
      }

      it {
        should raise_argument_error_for(
          "utf-u8 with\x00NUL byte",
          "utf-8").containing("contains embedded NUL bytes")
      }
    end
  end
end
