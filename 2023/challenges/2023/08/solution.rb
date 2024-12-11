# frozen_string_literal: true
module Year2023
  class Day08 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      instructions, adj_list = data
      position = "AAA"
      steps = 0
      while position != "ZZZ"
        instructions.each do |instruction|
          case instruction
          when "L"
            position = adj_list[position][:left]
          when "R"
            position = adj_list[position][:right]
          end
          steps += 1
          if position == "ZZZ"
            break
          end
        end
      end
      steps
    end

    def part_2
      instructions, adj_list = data
      all_a_positions = adj_list.keys.select { |position| position.end_with?("A") }
      steps = all_a_positions.map do |position|
        step = 0
        while !position.end_with?("Z")
          instructions.each do |instruction|
            case instruction
            when "L"
              position = adj_list[position][:left]
            when "R"
              position = adj_list[position][:right]
            end
            step += 1
            if position.end_with?("Z")
              break
            end
          end
        end
        step
      end
      steps.reduce { |lcm, num| lcm.lcm(num) }
    end

    private
      # Processes each line of the input file and stores the result in the dataset
      # def process_input(line)
      #   line.map(&:to_i)
      # end

      # Processes the dataset as a whole
      def process_dataset(set)
        instructions = []
        adj_list = {}
        set.each do |line|
          case line
          when /^[LR]+$+/
            instructions.concat(line.split(""))
          when /^([A-Z0-9]{3})\s=\s\(([A-Z0-9]{3}),\s([A-Z0-9]{3})\)$/
            adj_list[$1] = {:left => $2, :right => $3}
          else
            next
          end
        end
        [instructions, adj_list]
      end
  end
end
