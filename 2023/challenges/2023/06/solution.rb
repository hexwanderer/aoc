# frozen_string_literal: true
module Year2023
  class Day06 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      times, distances = data
      ways = times.zip(distances).map do |time, distance|
        pair(time, distance)
      end
      ways.reduce(:*)
    end

    def part_2
      times, distances = data
      time = times.map { |time| time.to_s }.join("").to_i
      distance = distances.map { |distance| distance.to_s }.join("").to_i
      pair(time, distance)
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      def pair(time, distance)
        (0...time).map do |i|
          holding, going = i, time - i
          holding * going
        end.map do |i|
          i > distance ? 1 : 0
        end.sum
      end

      # Processes the dataset as a whole
      def process_dataset(set)
        times, distances = set.map do |line|
          components = line.split(/\s+/)[1...].map(&:to_i)
          components
        end
        [times, distances]
      end
  end
end
