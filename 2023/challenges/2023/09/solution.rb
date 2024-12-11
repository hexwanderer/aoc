# frozen_string_literal: true
module Year2023
  class Day09 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.map do |line|
        extrapolate_forwards(line)
      end.sum
    end

    def part_2
      data.map do |line|
        extrapolate_backwards(line)
      end.sum
    end

    private
      def process_input(line)
        line.split(" ").map(&:to_i)
      end

      def difference(sequence)
        sequence.each_cons(2).map { |a, b| b - a }
      end

      def build_difference_table(original)
        table = [original.dup]
        while table.last.size > 1
          diffs = difference(table.last)
          table << diffs
          break if diffs.all?(&:zero?)
        end
        table
      end

      def extrapolate_forwards(original)
        table = build_difference_table(original)
        # Add a zero at the end of the bottom (all-zero) line
        table[-1] << 0

        # Reconstruct upwards
        # Start from the bottom and move up
        (table.size - 2).downto(0) do |level|
          # Each line above is reconstructed using the line below
          # forward: U[i+1] = U[i] + D[i]
          table[level] << (table[level].last + table[level + 1].last)
        end

        table[0].last
      end

      def extrapolate_backwards(original)
        table = build_difference_table(original)
        # Add a zero at the start of the bottom (all-zero) line
        table[-1].unshift(0)

        # Reconstruct upwards
        # For backward extrapolation, we do the mirror of forward:
        # forward formula: L[i+1] = L[i] + D[i]
        # backward formula: L[i] = L[i+1] - D[i]
        (table.size - 2).downto(0) do |level|
          new_first = table[level].first - table[level + 1].first
          table[level].unshift(new_first)
        end

        # After this reconstruction, the very top line (original sequence line) now has one extra element at the start.
        # That new first element is the backward extrapolated value.
        table[0].first
      end
  end
end
