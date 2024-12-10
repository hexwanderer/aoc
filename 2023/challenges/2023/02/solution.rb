# frozen_string_literal: true
module Year2023
  class Day02 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    MAX_PIECES = {
      "red" => 12,
      "green" => 13,
      "blue" => 14
    }

    def part_1
      data.map do |line|
        game_id, pieces = parse_game(line)

        valid = true
        pieces.each do |color, count|
          if count > MAX_PIECES[color]
            valid = false
            break
          end
        end
        valid ? game_id : 0
      end.sum
    end

    def part_2
      data.map do |line|
        _, pieces = parse_game(line)
        minimum = 1
        pieces.each do |color, count|
          minimum *= count
        end
        minimum
      end.sum
    end

    private
      def parse_game(line)
        parts = line.split(":")
        game_id = parts[0].split(" ")[1].to_i
        revealed_sets = parts[1].split(";")

        pieces = {}
        revealed_sets.each do |set|
          games = set.split(", ").map(&:strip)
          games.each do |game|
            color = game.split(" ")[1]
            count = game.split(" ")[0].to_i
            # max number of pieces for the color
            pieces[color] = [pieces[color] || 0, count].max
          end
        end
        [game_id, pieces]
      end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end
  end
end
