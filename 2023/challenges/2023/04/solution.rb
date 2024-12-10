# frozen_string_literal: true

require 'set'

module Year2023
  class Day04 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.map do |line|
        divided = line.split(" | ")
        winning_numbers = Set.new(divided[0].split(":")[1].split(" ").map(&:to_i))
        your_numbers = Set.new(divided[1].split(" ").map(&:to_i))
        intersection = winning_numbers & your_numbers
        intersection.size > 0 ? 2 ** (intersection.size - 1) : 0
      end.sum
    end

    def part_2
      copies = Hash[(0...data.size).map { |i| [i + 1, 1] }]
      data.map.with_index do |line, index|
        divided = line.split(" | ")
        winning_numbers = Set.new(divided[0].split(":")[1].split(" ").map(&:to_i))
        your_numbers = Set.new(divided[1].split(" ").map(&:to_i))
        intersection = winning_numbers & your_numbers
         # Snapshot the number of iterations to avoid modifying the range during iteration
        iterations = copies[index + 1]

        for _ in 0...iterations
          for i in 1..intersection.size
            copies[index + 1 + i] += 1
          end
        end
      end


      copies.values.sum
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
