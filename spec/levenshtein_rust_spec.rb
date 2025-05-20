# frozen_string_literal: true
RSpec::Matchers.define :measure_the_distance_between do |s1, s2|
  chain(:as) { |amount| @amount = amount }
  match { |klass| klass.distance(s1, s2) == @amount }

  failure_message do |klass|
    super() + ", but it was #{klass.distance(s1, s2)}"
  end
end

RSpec.describe LevenshteinRust do
  subject { LevenshteinRust }

  context '::VERSION' do
    it 'represents the version number' do
      expect(LevenshteinRust::VERSION).not_to be nil
    end
  end

  context '.distance' do
    it { should measure_the_distance_between('',  '' ).as(0) }
    it { should measure_the_distance_between('',  'a').as(1) }
    it { should measure_the_distance_between('a', '' ).as(1) }
    it { should measure_the_distance_between('a', 'b').as(1) }

    it { should measure_the_distance_between('abc', 'azc').as(1) }
    it { should measure_the_distance_between('abc', 'acb').as(2) }
    it { should measure_the_distance_between('abc', 'ac' ).as(1) }
    it { should measure_the_distance_between('ac' , 'abc').as(1) }

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
  end
end
