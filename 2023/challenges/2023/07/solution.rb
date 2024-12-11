# frozen_string_literal: true
module Year2023
  class Day07 < Solution
    # @input is available if you need the raw data input
    # Call `data` to access either an array of the parsed data, or a single record for a 1-line input file

    def part_1
      data.sort_by do |cards, hand, _, bid|
        [-WINNING_HANDS.index(hand), cards.map { |card| -WINNING_CARDS.values.index(card) }]
      end.map.with_index do |(cards, hand, _, bid), index|
        bid * (index + 1)
      end.sum
    end

    def part_2
      data.sort_by do |cards, _, hand, bid|
        [-WINNING_HANDS.index(hand), cards.map { |card| -WINNING_CARDS_IF_JOKER.values.index(card) }]
      end.map.with_index do |(cards, _, hand, bid), index|
        bid * (index + 1)
      end.sum
    end

    private
      WINNING_HANDS = [:five_of_a_kind, :four_of_a_kind, :full_house, :three_of_a_kind, :two_pair, :pair, :high_card]
      WINNING_CARDS = {
        "A" => :ace,
        "K" => :king,
        "Q" => :queen,
        "J" => :jack,
        "T" => :ten,
        "9" => :nine,
        "8" => :eight,
        "7" => :seven,
        "6" => :six,
        "5" => :five,
        "4" => :four,
        "3" => :three,
        "2" => :two,
      }

      WINNING_CARDS_IF_JOKER = {
        "A" => :ace,
        "K" => :king,
        "Q" => :queen,
        "T" => :ten,
        "9" => :nine,
        "8" => :eight,
        "7" => :seven,
        "6" => :six,
        "5" => :five,
        "4" => :four,
        "3" => :three,
        "2" => :two,
        "J" => :jack,
      }

      def get_hand(counts, joker = false)
        if !joker
          count_per_type = counts.values.sort
          highest = count_per_type.last
          second_highest = count_per_type[count_per_type.length - 2]
        else
          count_per_type = counts.select{ |k, v| k != :jack }.values.sort
          highest = (count_per_type.last || 0) + counts[:jack] || 0
          second_highest = count_per_type[count_per_type.length - 2]
        end

        if highest == 5
          :five_of_a_kind
        elsif highest == 4
          :four_of_a_kind
        elsif highest == 3 && second_highest == 2
          :full_house
        elsif highest == 3
          :three_of_a_kind
        elsif highest == 2 && second_highest == 2
          :two_pair
        elsif highest == 2
          :pair
        else
          :high_card
        end
      end

      # Processes each line of the input file and stores the result in the dataset
      def process_input(line)
        parts = line.split(" ")
        cards = parts[0].split("").map do |card|
          WINNING_CARDS[card]
        end
        bid = parts[1].to_i

        counts = Hash.new(0)
        cards.each do |card|
          counts[card] += 1
        end

        hand = get_hand(counts, false)
        hand_if_joker = get_hand(counts, true)

        [cards, hand, hand_if_joker, bid]
      end

      # Processes the dataset as a whole
      # def process_dataset(set)
      #   set
      # end
  end
end
