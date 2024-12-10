# frozen_string_literal: true
module Year2023
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      sums = 0
      data.each do |line|
        line = line.chomp
        pairs = []
        line.each_char do |char|
          if char.match?(/\d/)
            pairs << char
          end
        end

        left = pairs.first
        right = pairs.last

        combined = (left + right).to_i
        sums += combined
      end
      sums
    end

    DIGIT_MAP = {
      "one" => "1",
      "two" => "2",
      "three" => "3",
      "four" => "4",
      "five" => "5",
      "six" => "6",
      "seven" => "7",
      "eight" => "8",
      "nine" => "9"
    }

    def part_2
      sums = 0
      numeric_words = %w[one two three four five six seven eight nine]
      data.each do |line|
        line = line.chomp

        pairs = []
        line.each_char.with_index do |char, i|
          # Check if the character is a digit
          if char.match?(/\d/)
            pairs << char
            # Check if the substring is a numeric word
          elsif numeric_words.any? { |word| line[i..].start_with?(word) }
            word = numeric_words.find { |w| line[i..].start_with?(w) }
            pairs << DIGIT_MAP[word]
          end
        end

        left = pairs.first
        right = pairs.last

        combined = (left + right).to_i
        sums += combined
      end
      sums
    end

    private
    # Processes each line of the input file and stores the result in the dataset
    # def process_input(line)
    #   line.map(&:to_i)
    # end

    # Processes the dataset as a whole
    # def process_dataset(set)
    #   set
    # end
  end
end
