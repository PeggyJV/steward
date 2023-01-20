// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package integration_tests

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
)

// CellarAdaptorCall is an auto generated low-level Go binding around an user-defined struct.
type CellarAdaptorCall struct {
	Adaptor  common.Address
	CallData [][]byte
}

// CellarMetaData contains all meta data concerning the Cellar contract.
var CellarMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"adaptor\",\"type\":\"address\"},{\"internalType\":\"bytes[]\",\"name\":\"callData\",\"type\":\"bytes[]\"}],\"indexed\":false,\"internalType\":\"structCellar.AdaptorCall[]\",\"name\":\"data\",\"type\":\"tuple[]\"}],\"name\":\"CallOnAdaptor\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"fromPosition\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"toPosition\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"assetsFrom\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"}],\"name\":\"Rebalance\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"position\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isTrusted\",\"type\":\"bool\"}],\"name\":\"TrustChanged\",\"type\":\"event\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"adaptor\",\"type\":\"address\"},{\"internalType\":\"bytes[]\",\"name\":\"callData\",\"type\":\"bytes[]\"}],\"internalType\":\"structCellar.AdaptorCall[]\",\"name\":\"data\",\"type\":\"tuple[]\"}],\"name\":\"callOnAdaptor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromPosition\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"toPosition\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"assetsFrom\",\"type\":\"uint256\"},{\"internalType\":\"enumCellar.Exchange\",\"name\":\"exchange\",\"type\":\"uint8\"},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\"}],\"name\":\"rebalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"assetsTo\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"position\",\"type\":\"address\"},{\"internalType\":\"enumCellar.PositionType\",\"name\":\"positionType\",\"type\":\"uint8\"}],\"name\":\"trustPosition\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// CellarABI is the input ABI used to generate the binding from.
// Deprecated: Use CellarMetaData.ABI instead.
var CellarABI = CellarMetaData.ABI

// Cellar is an auto generated Go binding around an Ethereum contract.
type Cellar struct {
	CellarCaller     // Read-only binding to the contract
	CellarTransactor // Write-only binding to the contract
	CellarFilterer   // Log filterer for contract events
}

// CellarCaller is an auto generated read-only Go binding around an Ethereum contract.
type CellarCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarTransactor is an auto generated write-only Go binding around an Ethereum contract.
type CellarTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type CellarFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type CellarSession struct {
	Contract     *Cellar           // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type CellarCallerSession struct {
	Contract *CellarCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// CellarTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type CellarTransactorSession struct {
	Contract     *CellarTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarRaw is an auto generated low-level Go binding around an Ethereum contract.
type CellarRaw struct {
	Contract *Cellar // Generic contract binding to access the raw methods on
}

// CellarCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type CellarCallerRaw struct {
	Contract *CellarCaller // Generic read-only contract binding to access the raw methods on
}

// CellarTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type CellarTransactorRaw struct {
	Contract *CellarTransactor // Generic write-only contract binding to access the raw methods on
}

// NewCellar creates a new instance of Cellar, bound to a specific deployed contract.
func NewCellar(address common.Address, backend bind.ContractBackend) (*Cellar, error) {
	contract, err := bindCellar(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Cellar{CellarCaller: CellarCaller{contract: contract}, CellarTransactor: CellarTransactor{contract: contract}, CellarFilterer: CellarFilterer{contract: contract}}, nil
}

// NewCellarCaller creates a new read-only instance of Cellar, bound to a specific deployed contract.
func NewCellarCaller(address common.Address, caller bind.ContractCaller) (*CellarCaller, error) {
	contract, err := bindCellar(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &CellarCaller{contract: contract}, nil
}

// NewCellarTransactor creates a new write-only instance of Cellar, bound to a specific deployed contract.
func NewCellarTransactor(address common.Address, transactor bind.ContractTransactor) (*CellarTransactor, error) {
	contract, err := bindCellar(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &CellarTransactor{contract: contract}, nil
}

// NewCellarFilterer creates a new log filterer instance of Cellar, bound to a specific deployed contract.
func NewCellarFilterer(address common.Address, filterer bind.ContractFilterer) (*CellarFilterer, error) {
	contract, err := bindCellar(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &CellarFilterer{contract: contract}, nil
}

// bindCellar binds a generic wrapper to an already deployed contract.
func bindCellar(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(CellarABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Cellar *CellarRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Cellar.Contract.CellarCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Cellar *CellarRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Cellar.Contract.CellarTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Cellar *CellarRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Cellar.Contract.CellarTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Cellar *CellarCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Cellar.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Cellar *CellarTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Cellar.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Cellar *CellarTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Cellar.Contract.contract.Transact(opts, method, params...)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Cellar *CellarCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Cellar.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Cellar *CellarSession) Owner() (common.Address, error) {
	return _Cellar.Contract.Owner(&_Cellar.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_Cellar *CellarCallerSession) Owner() (common.Address, error) {
	return _Cellar.Contract.Owner(&_Cellar.CallOpts)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_Cellar *CellarTransactor) CallOnAdaptor(opts *bind.TransactOpts, data []CellarAdaptorCall) (*types.Transaction, error) {
	return _Cellar.contract.Transact(opts, "callOnAdaptor", data)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_Cellar *CellarSession) CallOnAdaptor(data []CellarAdaptorCall) (*types.Transaction, error) {
	return _Cellar.Contract.CallOnAdaptor(&_Cellar.TransactOpts, data)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_Cellar *CellarTransactorSession) CallOnAdaptor(data []CellarAdaptorCall) (*types.Transaction, error) {
	return _Cellar.Contract.CallOnAdaptor(&_Cellar.TransactOpts, data)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarTransactor) Rebalance(opts *bind.TransactOpts, fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.contract.Transact(opts, "rebalance", fromPosition, toPosition, assetsFrom, exchange, params)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarSession) Rebalance(fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.Contract.Rebalance(&_Cellar.TransactOpts, fromPosition, toPosition, assetsFrom, exchange, params)
}

// Rebalance is a paid mutator transaction binding the contract method 0x389a7294.
//
// Solidity: function rebalance(address fromPosition, address toPosition, uint256 assetsFrom, uint8 exchange, bytes params) returns(uint256 assetsTo)
func (_Cellar *CellarTransactorSession) Rebalance(fromPosition common.Address, toPosition common.Address, assetsFrom *big.Int, exchange uint8, params []byte) (*types.Transaction, error) {
	return _Cellar.Contract.Rebalance(&_Cellar.TransactOpts, fromPosition, toPosition, assetsFrom, exchange, params)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Cellar *CellarTransactor) SetOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _Cellar.contract.Transact(opts, "setOwner", newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Cellar *CellarSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Cellar.Contract.SetOwner(&_Cellar.TransactOpts, newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_Cellar *CellarTransactorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _Cellar.Contract.SetOwner(&_Cellar.TransactOpts, newOwner)
}

// TrustPosition is a paid mutator transaction binding the contract method 0xfc4d43be.
//
// Solidity: function trustPosition(address position, uint8 positionType) returns()
func (_Cellar *CellarTransactor) TrustPosition(opts *bind.TransactOpts, position common.Address, positionType uint8) (*types.Transaction, error) {
	return _Cellar.contract.Transact(opts, "trustPosition", position, positionType)
}

// TrustPosition is a paid mutator transaction binding the contract method 0xfc4d43be.
//
// Solidity: function trustPosition(address position, uint8 positionType) returns()
func (_Cellar *CellarSession) TrustPosition(position common.Address, positionType uint8) (*types.Transaction, error) {
	return _Cellar.Contract.TrustPosition(&_Cellar.TransactOpts, position, positionType)
}

// TrustPosition is a paid mutator transaction binding the contract method 0xfc4d43be.
//
// Solidity: function trustPosition(address position, uint8 positionType) returns()
func (_Cellar *CellarTransactorSession) TrustPosition(position common.Address, positionType uint8) (*types.Transaction, error) {
	return _Cellar.Contract.TrustPosition(&_Cellar.TransactOpts, position, positionType)
}

// CellarCallOnAdaptorIterator is returned from FilterCallOnAdaptor and is used to iterate over the raw logs and unpacked data for CallOnAdaptor events raised by the Cellar contract.
type CellarCallOnAdaptorIterator struct {
	Event *CellarCallOnAdaptor // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *CellarCallOnAdaptorIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarCallOnAdaptor)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(CellarCallOnAdaptor)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *CellarCallOnAdaptorIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarCallOnAdaptorIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarCallOnAdaptor represents a CallOnAdaptor event raised by the Cellar contract.
type CellarCallOnAdaptor struct {
	Data []CellarAdaptorCall
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterCallOnAdaptor is a free log retrieval operation binding the contract event 0x261a6cb604cad99de116029b1ff284d989beb85ed58cd0f9042dd57363c04303.
//
// Solidity: event CallOnAdaptor((address,bytes[])[] data)
func (_Cellar *CellarFilterer) FilterCallOnAdaptor(opts *bind.FilterOpts) (*CellarCallOnAdaptorIterator, error) {

	logs, sub, err := _Cellar.contract.FilterLogs(opts, "CallOnAdaptor")
	if err != nil {
		return nil, err
	}
	return &CellarCallOnAdaptorIterator{contract: _Cellar.contract, event: "CallOnAdaptor", logs: logs, sub: sub}, nil
}

// WatchCallOnAdaptor is a free log subscription operation binding the contract event 0x261a6cb604cad99de116029b1ff284d989beb85ed58cd0f9042dd57363c04303.
//
// Solidity: event CallOnAdaptor((address,bytes[])[] data)
func (_Cellar *CellarFilterer) WatchCallOnAdaptor(opts *bind.WatchOpts, sink chan<- *CellarCallOnAdaptor) (event.Subscription, error) {

	logs, sub, err := _Cellar.contract.WatchLogs(opts, "CallOnAdaptor")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarCallOnAdaptor)
				if err := _Cellar.contract.UnpackLog(event, "CallOnAdaptor", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseCallOnAdaptor is a log parse operation binding the contract event 0x261a6cb604cad99de116029b1ff284d989beb85ed58cd0f9042dd57363c04303.
//
// Solidity: event CallOnAdaptor((address,bytes[])[] data)
func (_Cellar *CellarFilterer) ParseCallOnAdaptor(log types.Log) (*CellarCallOnAdaptor, error) {
	event := new(CellarCallOnAdaptor)
	if err := _Cellar.contract.UnpackLog(event, "CallOnAdaptor", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarOwnerUpdatedIterator is returned from FilterOwnerUpdated and is used to iterate over the raw logs and unpacked data for OwnerUpdated events raised by the Cellar contract.
type CellarOwnerUpdatedIterator struct {
	Event *CellarOwnerUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *CellarOwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarOwnerUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(CellarOwnerUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *CellarOwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarOwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarOwnerUpdated represents a OwnerUpdated event raised by the Cellar contract.
type CellarOwnerUpdated struct {
	User     common.Address
	NewOwner common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOwnerUpdated is a free log retrieval operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Cellar *CellarFilterer) FilterOwnerUpdated(opts *bind.FilterOpts, user []common.Address, newOwner []common.Address) (*CellarOwnerUpdatedIterator, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Cellar.contract.FilterLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &CellarOwnerUpdatedIterator{contract: _Cellar.contract, event: "OwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchOwnerUpdated is a free log subscription operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Cellar *CellarFilterer) WatchOwnerUpdated(opts *bind.WatchOpts, sink chan<- *CellarOwnerUpdated, user []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _Cellar.contract.WatchLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarOwnerUpdated)
				if err := _Cellar.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnerUpdated is a log parse operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_Cellar *CellarFilterer) ParseOwnerUpdated(log types.Log) (*CellarOwnerUpdated, error) {
	event := new(CellarOwnerUpdated)
	if err := _Cellar.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarRebalanceIterator is returned from FilterRebalance and is used to iterate over the raw logs and unpacked data for Rebalance events raised by the Cellar contract.
type CellarRebalanceIterator struct {
	Event *CellarRebalance // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *CellarRebalanceIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarRebalance)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(CellarRebalance)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *CellarRebalanceIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarRebalanceIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarRebalance represents a Rebalance event raised by the Cellar contract.
type CellarRebalance struct {
	FromPosition common.Address
	ToPosition   common.Address
	AssetsFrom   *big.Int
	Exchange     uint8
	Raw          types.Log // Blockchain specific contextual infos
}

// FilterRebalance is a free log retrieval operation binding the contract event 0x8692b9960dc9923bf97ab45a1eda03230fbcbf8d8ca8f9c9712b0cb8e0599571.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom, uint8 exchange)
func (_Cellar *CellarFilterer) FilterRebalance(opts *bind.FilterOpts, fromPosition []common.Address, toPosition []common.Address) (*CellarRebalanceIterator, error) {

	var fromPositionRule []interface{}
	for _, fromPositionItem := range fromPosition {
		fromPositionRule = append(fromPositionRule, fromPositionItem)
	}
	var toPositionRule []interface{}
	for _, toPositionItem := range toPosition {
		toPositionRule = append(toPositionRule, toPositionItem)
	}

	logs, sub, err := _Cellar.contract.FilterLogs(opts, "Rebalance", fromPositionRule, toPositionRule)
	if err != nil {
		return nil, err
	}
	return &CellarRebalanceIterator{contract: _Cellar.contract, event: "Rebalance", logs: logs, sub: sub}, nil
}

// WatchRebalance is a free log subscription operation binding the contract event 0x8692b9960dc9923bf97ab45a1eda03230fbcbf8d8ca8f9c9712b0cb8e0599571.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom, uint8 exchange)
func (_Cellar *CellarFilterer) WatchRebalance(opts *bind.WatchOpts, sink chan<- *CellarRebalance, fromPosition []common.Address, toPosition []common.Address) (event.Subscription, error) {

	var fromPositionRule []interface{}
	for _, fromPositionItem := range fromPosition {
		fromPositionRule = append(fromPositionRule, fromPositionItem)
	}
	var toPositionRule []interface{}
	for _, toPositionItem := range toPosition {
		toPositionRule = append(toPositionRule, toPositionItem)
	}

	logs, sub, err := _Cellar.contract.WatchLogs(opts, "Rebalance", fromPositionRule, toPositionRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarRebalance)
				if err := _Cellar.contract.UnpackLog(event, "Rebalance", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRebalance is a log parse operation binding the contract event 0x8692b9960dc9923bf97ab45a1eda03230fbcbf8d8ca8f9c9712b0cb8e0599571.
//
// Solidity: event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom, uint8 exchange)
func (_Cellar *CellarFilterer) ParseRebalance(log types.Log) (*CellarRebalance, error) {
	event := new(CellarRebalance)
	if err := _Cellar.contract.UnpackLog(event, "Rebalance", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarTrustChangedIterator is returned from FilterTrustChanged and is used to iterate over the raw logs and unpacked data for TrustChanged events raised by the Cellar contract.
type CellarTrustChangedIterator struct {
	Event *CellarTrustChanged // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *CellarTrustChangedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarTrustChanged)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(CellarTrustChanged)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *CellarTrustChangedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarTrustChangedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarTrustChanged represents a TrustChanged event raised by the Cellar contract.
type CellarTrustChanged struct {
	Position  common.Address
	IsTrusted bool
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterTrustChanged is a free log retrieval operation binding the contract event 0xd600b9348603c6deff34b4e0b28b60e1c8036c806741b9e6d90032e7f37dd27f.
//
// Solidity: event TrustChanged(address position, bool isTrusted)
func (_Cellar *CellarFilterer) FilterTrustChanged(opts *bind.FilterOpts) (*CellarTrustChangedIterator, error) {

	logs, sub, err := _Cellar.contract.FilterLogs(opts, "TrustChanged")
	if err != nil {
		return nil, err
	}
	return &CellarTrustChangedIterator{contract: _Cellar.contract, event: "TrustChanged", logs: logs, sub: sub}, nil
}

// WatchTrustChanged is a free log subscription operation binding the contract event 0xd600b9348603c6deff34b4e0b28b60e1c8036c806741b9e6d90032e7f37dd27f.
//
// Solidity: event TrustChanged(address position, bool isTrusted)
func (_Cellar *CellarFilterer) WatchTrustChanged(opts *bind.WatchOpts, sink chan<- *CellarTrustChanged) (event.Subscription, error) {

	logs, sub, err := _Cellar.contract.WatchLogs(opts, "TrustChanged")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarTrustChanged)
				if err := _Cellar.contract.UnpackLog(event, "TrustChanged", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseTrustChanged is a log parse operation binding the contract event 0xd600b9348603c6deff34b4e0b28b60e1c8036c806741b9e6d90032e7f37dd27f.
//
// Solidity: event TrustChanged(address position, bool isTrusted)
func (_Cellar *CellarFilterer) ParseTrustChanged(log types.Log) (*CellarTrustChanged, error) {
	event := new(CellarTrustChanged)
	if err := _Cellar.contract.UnpackLog(event, "TrustChanged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
