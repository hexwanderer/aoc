# frozen_string_literal: true
module Year2023
  class Day05 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    module Mode
      SEED_SOIL = 0
      SOIL_FERTILIZER = 1
      FERTILIZIER_WATER = 2
      WATER_LIGHT = 3
      LIGHT_TEMPERATURE = 4
      TEMPERATURE_HUMIDITY = 5
      HUMIDITY_LOCATION = 6
    end

    def part_1
      lines = @input.lines.map(&:chomp)
      seeds, mode_maps = process_dataset(lines)

      seeds.map do |seed|
        process_seed(seed, mode_maps)
      end.min
    end

    def part_2
      lines = @input.lines.map(&:chomp)
      seeds, mode_maps = process_dataset(lines)
      seeds.each_slice(2).map do |left, right|
        (left...left+right).map do |seed|
          puts "seed #{seed}"
          result = process_seed(seed, mode_maps)
          result
        end.min
      end.min
    end

    private
      def fully_encompass(range1, range2)
        if range1.begin <= range2.begin && range1.end >= range2.end
          :full
        elsif range1.begin <= range2.begin || range1.end >= range2.end
          :partial[range1, range2]
        else
          :none
        end
      end

      # Processes each line of the input file and stores the result in the dataset
      def process_seed(seed, mode_maps)
        current = seed
        (0..6).each do |mode|
          next if mode_maps[mode].nil?
          mode_maps[mode].each do |range, value|
            if range.include?(current)
              current += value
              break
            end
          end
        end
        current
      end

      def process_range(current_range, mode_maps, current_mode)
        if current_mode == 6
          current_range.min
        end
        node_maps[current_mode].each do |range, value|
          case fully_encompass(current_range, range)
          when :full
            current_range = range
            return process_range(current_range, mode_maps, current_mode + 1)
          when :partial
            range_1, range_2 = 0..0, 0..0
            return [process_range(range_1, mode_maps, current_mode + 1), process_range(range_2, mode_maps, current_mode + 1)].min
          when :none
            continue
          end
        end
      end

      # Processes the dataset as a whole
      def process_dataset(lines)
        mode = nil
        seeds = []
        mode_maps = {}
        lines.map do |line|
          case line
          when /^seeds:/
            seeds.concat(line.split(":")[1].strip.split(" ").map(&:to_i))
          when /^seed-to-soil map:/
            mode = Mode::SEED_SOIL
          when /^soil-to-fertilizer map:/
            mode = Mode::SOIL_FERTILIZER
          when /^fertilizer-to-water map:/
            mode = Mode::FERTILIZIER_WATER
          when /^water-to-light map:/
            mode = Mode::WATER_LIGHT
          when /^light-to-temperature map:/
            mode = Mode::LIGHT_TEMPERATURE
          when /^temperature-to-humidity map:/
            mode = Mode::TEMPERATURE_HUMIDITY
          when /^humidity-to-location map:/
            mode = Mode::HUMIDITY_LOCATION
          else
            next if line.length == 0
            components = line.split(" ").map(&:to_i)
            right, left, range = components[0], components[1], components[2]
            left_range = left...left+range
            mode_maps[mode] ||= {}
            mode_maps[mode][left_range] = (right-left)
          end
        end
        [seeds, mode_maps]
      end
  end
end
