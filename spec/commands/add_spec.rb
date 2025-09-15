require 'spec_helper'
require_relative '../../src/commands/add'
require 'tempfile'

RSpec.describe TodoCli::Commands::Add do
  let(:temp_db) { Tempfile.new('test_todos.db') }

  before do
    # Mock the database file path to use a temporary file
    stub_const('TodoCli::Utils::Database::DB_FILE', temp_db.path)
  end

  after do
    temp_db.unlink
  end

  describe '#execute' do
    it 'adds a todo with default priority' do
      command = described_class.new('Buy groceries')

      expect { command.execute }.to output(/✓ Added todo #1: Buy groceries \[medium\]/).to_stdout
    end

    it 'adds a todo with high priority' do
      command = described_class.new('Urgent task', priority: 'high')

      expect { command.execute }.to output(/✓ Added todo #1: Urgent task \[high\]/).to_stdout
    end

    it 'returns a successful result object' do
      command = described_class.new('Test todo')

      result = nil
      capture_stdout { result = command.execute }

      expect(result).to be_success
      expect(result.message).to include('Todo added with ID')
    end

    it 'handles database errors gracefully' do
      command = described_class.new('Test todo')

      # Mock the database to raise an error
      allow(TodoCli::Utils::Database).to receive(:new).and_raise(StandardError.new('Database connection failed'))

      result = nil
      capture_stdout { result = command.execute }

      expect(result.success?).to be false
      expect(result.message).to include('Database connection failed')
    end
  end
end
