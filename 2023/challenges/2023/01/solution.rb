# frozen_string_literal: true
module Year2023
  class Day01 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.map do |line|
        first_num = line.find { |item| item[:type] == :num }
        last_num = line.reverse.find { |item| item[:type] == :num }
        first_num[:value] * 10 + last_num[:value]
      end.sum
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
      data.map do |line|
        first_num = line.first[:value]
        last_num = line.last[:value]
        first_num * 10 + last_num
      end.sum
    end

    private
      # Processes the dataset as a whole
      def process_dataset(set)
        set.map do |line|
          process_line(line)
        end
      end

      def process_line(line)
        result = []
        i = 0

        while i < line.length
          char = line[i]

          if char.match?(/\d/) # Match digits
            result << { type: :num, value: char.to_i }
          elsif DIGIT_MAP.keys.any? { |word| line[i..].start_with?(word) }
            word = DIGIT_MAP.keys.find { |w| line[i..].start_with?(w) }
            result << { type: :word, value: DIGIT_MAP[word].to_i }
          end
          i += 1
        end

        result
      end
  end
end
