# frozen_string_literal: true
require 'spec_helper'

RSpec.describe Year2023::Day08 do
  let(:input) { File.read(File.join(File.dirname(__FILE__), "../../../challenges/2023/08/input.txt")) }
  let(:example_input) {
    <<~EOF
      RL

      AAA = (BBB, CCC)
      BBB = (DDD, EEE)
      CCC = (ZZZ, GGG)
      DDD = (DDD, DDD)
      EEE = (EEE, EEE)
      GGG = (GGG, GGG)
      ZZZ = (ZZZ, ZZZ)
    EOF
  }

  describe "part 1" do
    it "returns 2 for the example input" do
      expect(described_class.part_1(example_input)).to eq(2)
    end

    it "returns nil for my input" do
      expect(described_class.part_1(input)).not_to eq(nil)
    end
  end

  describe "part 2" do
    it "returns 2 for the example input" do
      expect(described_class.part_2(example_input)).to eq(2)
    end

    it "returns nil for my input" do
      expect(described_class.part_2(input)).not_to eq(nil)
    end
  end
end
