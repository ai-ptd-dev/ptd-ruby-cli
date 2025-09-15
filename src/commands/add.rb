require_relative '../utils/database'

module TodoCli
  module Commands
    class Add
      def initialize(text, options = {})
        @text = text
        @priority = options[:priority] || 'medium'
      end

      def execute
        db = Utils::Database.new
        id = db.add_todo(@text, @priority)
        puts "✓ Added todo ##{id}: #{@text} [#{@priority}]"
        Result.new(success: true, message: "Todo added with ID #{id}")
      rescue StandardError => e
        Result.new(success: false, message: e.message)
      ensure
        db&.close
      end

      class Result
        attr_reader :success, :message

        def initialize(success:, message:)
          @success = success
          @message = message
        end

        def success?
          @success
        end
      end
    end
  end
end
