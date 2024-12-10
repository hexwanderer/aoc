# frozen_string_literal: true
module Year2023
  class Day03 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      numbers, max_row, max_col = build_schematic(data)

      # any non-alpha, non-number, non-period characters are symbols
      valid_numbers = []
      numbers.each do |range, number|
        adjacent(range[0], range[1], range[2], max_row, max_col).each do |coord|
          next if coord.nil?
          row, col = coord
          if !data[row][col].match?(/\d/) && data[row][col] != "."
            valid_numbers << number.to_i
            break
          end
        end
      end

      valid_numbers.sum
    end

    def part_2
      numbers, max_row, max_col = build_schematic(data)

      gears = {}
      numbers.each do |range, number|
        adjacent(range[0], range[1], range[2], max_row, max_col).each do |coord|
          next if coord.nil?
          row, col = coord
          next if data[row][col] != "*"

          gears[[row, col]] ||= []
          gears[[row, col]] << number.to_i
        end
      end

      gears.select { |_, v| v.length == 2 }.map { |k, v| v.first * v.last }.sum
    end

    private
      def build_schematic(input)
        numbers = {}
        is_number = false
        curr_number = ""

        # Get max row and column
        max_row = data.length
        max_col = data.first.length

        data.each.with_index do |line, r|
          line.each_char.with_index do |char, c|
            if !is_number && char.match?(/\d/)
              is_number = true
              curr_number += char
            elsif is_number && char.match?(/\d/)
              curr_number += char
            # if not a number, and we're in a number, save the number
            elsif is_number && !char.match?(/\d/)
              numbers[[r, c - curr_number.length, c - 1]] = curr_number
              is_number = false
              curr_number = ""
            end
          end
          if is_number
            numbers[[r, max_col - curr_number.length, max_col - 1]] = curr_number
            curr_number = ""
            is_number = false
          end
        end
        [numbers, max_row, max_col]
      end

      # Generate a list of all adjacent coordinates to a number,
      # where a number is represented by the row, and the start
      # and end columns
      def adjacent(x, y1, y2, max_row, max_col)
        adjacent_coordinates = []

        # Loop through the range of rows and columns to find adjacent coordinates
        (x - 1..x + 1).each do |row|
          next if row < 0 || row >= max_row # Ensure row is within bounds

          ((y1 - 1)..(y2 + 1)).each do |col|
            next if col < 0 || col >= max_col # Ensure column is within bounds
            next if row == x && (col >= y1 && col <= y2) # Skip the original range

            adjacent_coordinates << [row, col]
          end
        end

        adjacent_coordinates
      end
  end
end
