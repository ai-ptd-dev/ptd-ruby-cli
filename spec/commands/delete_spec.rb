require 'spec_helper'
require_relative '../../src/commands/delete'
require_relative '../../src/commands/add'
require 'tempfile'

RSpec.describe TodoCli::Commands::Delete do
  let(:temp_db) { Tempfile.new('test_todos.db') }

  before do
    # Mock the database file path to use a temporary file
    stub_const('TodoCli::Utils::Database::DB_FILE', temp_db.path)
  end

  after do
    temp_db.unlink
  end

  describe '#execute' do
    context 'when todo exists' do
      before do
        # Add a test todo
        capture_stdout { TodoCli::Commands::Add.new('Buy groceries').execute }
      end

      it 'deletes the todo successfully' do
        command = described_class.new('1')

        expect { command.execute }.to output(/✗ Deleted todo #1: Buy groceries/).to_stdout
      end

      it 'returns a successful result object' do
        command = described_class.new('1')

        result = nil
        capture_stdout { result = command.execute }

        expect(result).to be_success
        expect(result.message).to eq('Todo deleted')
      end
    end

    context 'when todo does not exist' do
      it 'shows todo not found message' do
        command = described_class.new('999')

        expect { command.execute }.to output(/Todo #999 not found/).to_stdout
      end

      it 'returns unsuccessful result object' do
        command = described_class.new('999')

        result = nil
        capture_stdout { result = command.execute }

        expect(result.success?).to be false
        expect(result.message).to eq('Todo not found')
      end
    end
  end
end
