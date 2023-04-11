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

// CellarV2_2AdaptorCall is an auto generated low-level Go binding around an user-defined struct.
type CellarV2_2AdaptorCall struct {
	Adaptor  common.Address
	CallData [][]byte
}

// CellarV22MetaData contains all meta data concerning the CellarV22 contract.
var CellarV22MetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"adaptor\",\"type\":\"address\"},{\"internalType\":\"bytes[]\",\"name\":\"callData\",\"type\":\"bytes[]\"}],\"indexed\":false,\"internalType\":\"structCellarV2_2.AdaptorCall[]\",\"name\":\"data\",\"type\":\"tuple[]\"}],\"name\":\"CallOnAdaptor\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Multicall\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerUpdated\",\"type\":\"event\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"adaptor\",\"type\":\"address\"},{\"internalType\":\"bytes[]\",\"name\":\"callData\",\"type\":\"bytes[]\"}],\"internalType\":\"structCellarV2_2.AdaptorCall[]\",\"name\":\"data\",\"type\":\"tuple[]\"}],\"name\":\"callOnAdaptor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\"}],\"name\":\"multicall\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// CellarV22ABI is the input ABI used to generate the binding from.
// Deprecated: Use CellarV22MetaData.ABI instead.
var CellarV22ABI = CellarV22MetaData.ABI

// CellarV22 is an auto generated Go binding around an Ethereum contract.
type CellarV22 struct {
	CellarV22Caller     // Read-only binding to the contract
	CellarV22Transactor // Write-only binding to the contract
	CellarV22Filterer   // Log filterer for contract events
}

// CellarV22Caller is an auto generated read-only Go binding around an Ethereum contract.
type CellarV22Caller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV22Transactor is an auto generated write-only Go binding around an Ethereum contract.
type CellarV22Transactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV22Filterer is an auto generated log filtering Go binding around an Ethereum contract events.
type CellarV22Filterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CellarV22Session is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type CellarV22Session struct {
	Contract     *CellarV22        // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CellarV22CallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type CellarV22CallerSession struct {
	Contract *CellarV22Caller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts    // Call options to use throughout this session
}

// CellarV22TransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type CellarV22TransactorSession struct {
	Contract     *CellarV22Transactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts    // Transaction auth options to use throughout this session
}

// CellarV22Raw is an auto generated low-level Go binding around an Ethereum contract.
type CellarV22Raw struct {
	Contract *CellarV22 // Generic contract binding to access the raw methods on
}

// CellarV22CallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type CellarV22CallerRaw struct {
	Contract *CellarV22Caller // Generic read-only contract binding to access the raw methods on
}

// CellarV22TransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type CellarV22TransactorRaw struct {
	Contract *CellarV22Transactor // Generic write-only contract binding to access the raw methods on
}

// NewCellarV22 creates a new instance of CellarV22, bound to a specific deployed contract.
func NewCellarV22(address common.Address, backend bind.ContractBackend) (*CellarV22, error) {
	contract, err := bindCellarV22(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &CellarV22{CellarV22Caller: CellarV22Caller{contract: contract}, CellarV22Transactor: CellarV22Transactor{contract: contract}, CellarV22Filterer: CellarV22Filterer{contract: contract}}, nil
}

// NewCellarV22Caller creates a new read-only instance of CellarV22, bound to a specific deployed contract.
func NewCellarV22Caller(address common.Address, caller bind.ContractCaller) (*CellarV22Caller, error) {
	contract, err := bindCellarV22(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &CellarV22Caller{contract: contract}, nil
}

// NewCellarV22Transactor creates a new write-only instance of CellarV22, bound to a specific deployed contract.
func NewCellarV22Transactor(address common.Address, transactor bind.ContractTransactor) (*CellarV22Transactor, error) {
	contract, err := bindCellarV22(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &CellarV22Transactor{contract: contract}, nil
}

// NewCellarV22Filterer creates a new log filterer instance of CellarV22, bound to a specific deployed contract.
func NewCellarV22Filterer(address common.Address, filterer bind.ContractFilterer) (*CellarV22Filterer, error) {
	contract, err := bindCellarV22(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &CellarV22Filterer{contract: contract}, nil
}

// bindCellarV22 binds a generic wrapper to an already deployed contract.
func bindCellarV22(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(CellarV22ABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_CellarV22 *CellarV22Raw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _CellarV22.Contract.CellarV22Caller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_CellarV22 *CellarV22Raw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _CellarV22.Contract.CellarV22Transactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_CellarV22 *CellarV22Raw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _CellarV22.Contract.CellarV22Transactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_CellarV22 *CellarV22CallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _CellarV22.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_CellarV22 *CellarV22TransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _CellarV22.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_CellarV22 *CellarV22TransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _CellarV22.Contract.contract.Transact(opts, method, params...)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV22 *CellarV22Caller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _CellarV22.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV22 *CellarV22Session) Owner() (common.Address, error) {
	return _CellarV22.Contract.Owner(&_CellarV22.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_CellarV22 *CellarV22CallerSession) Owner() (common.Address, error) {
	return _CellarV22.Contract.Owner(&_CellarV22.CallOpts)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_CellarV22 *CellarV22Transactor) CallOnAdaptor(opts *bind.TransactOpts, data []CellarV2_2AdaptorCall) (*types.Transaction, error) {
	return _CellarV22.contract.Transact(opts, "callOnAdaptor", data)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_CellarV22 *CellarV22Session) CallOnAdaptor(data []CellarV2_2AdaptorCall) (*types.Transaction, error) {
	return _CellarV22.Contract.CallOnAdaptor(&_CellarV22.TransactOpts, data)
}

// CallOnAdaptor is a paid mutator transaction binding the contract method 0x4e84befe.
//
// Solidity: function callOnAdaptor((address,bytes[])[] data) returns()
func (_CellarV22 *CellarV22TransactorSession) CallOnAdaptor(data []CellarV2_2AdaptorCall) (*types.Transaction, error) {
	return _CellarV22.Contract.CallOnAdaptor(&_CellarV22.TransactOpts, data)
}

// Multicall is a paid mutator transaction binding the contract method 0xac9650d8.
//
// Solidity: function multicall(bytes[] data) returns()
func (_CellarV22 *CellarV22Transactor) Multicall(opts *bind.TransactOpts, data [][]byte) (*types.Transaction, error) {
	return _CellarV22.contract.Transact(opts, "multicall", data)
}

// Multicall is a paid mutator transaction binding the contract method 0xac9650d8.
//
// Solidity: function multicall(bytes[] data) returns()
func (_CellarV22 *CellarV22Session) Multicall(data [][]byte) (*types.Transaction, error) {
	return _CellarV22.Contract.Multicall(&_CellarV22.TransactOpts, data)
}

// Multicall is a paid mutator transaction binding the contract method 0xac9650d8.
//
// Solidity: function multicall(bytes[] data) returns()
func (_CellarV22 *CellarV22TransactorSession) Multicall(data [][]byte) (*types.Transaction, error) {
	return _CellarV22.Contract.Multicall(&_CellarV22.TransactOpts, data)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV22 *CellarV22Transactor) SetOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _CellarV22.contract.Transact(opts, "setOwner", newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV22 *CellarV22Session) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _CellarV22.Contract.SetOwner(&_CellarV22.TransactOpts, newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_CellarV22 *CellarV22TransactorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _CellarV22.Contract.SetOwner(&_CellarV22.TransactOpts, newOwner)
}

// CellarV22CallOnAdaptorIterator is returned from FilterCallOnAdaptor and is used to iterate over the raw logs and unpacked data for CallOnAdaptor events raised by the CellarV22 contract.
type CellarV22CallOnAdaptorIterator struct {
	Event *CellarV22CallOnAdaptor // Event containing the contract specifics and raw log

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
func (it *CellarV22CallOnAdaptorIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarV22CallOnAdaptor)
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
		it.Event = new(CellarV22CallOnAdaptor)
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
func (it *CellarV22CallOnAdaptorIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarV22CallOnAdaptorIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarV22CallOnAdaptor represents a CallOnAdaptor event raised by the CellarV22 contract.
type CellarV22CallOnAdaptor struct {
	Data []CellarV2_2AdaptorCall
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterCallOnAdaptor is a free log retrieval operation binding the contract event 0x261a6cb604cad99de116029b1ff284d989beb85ed58cd0f9042dd57363c04303.
//
// Solidity: event CallOnAdaptor((address,bytes[])[] data)
func (_CellarV22 *CellarV22Filterer) FilterCallOnAdaptor(opts *bind.FilterOpts) (*CellarV22CallOnAdaptorIterator, error) {

	logs, sub, err := _CellarV22.contract.FilterLogs(opts, "CallOnAdaptor")
	if err != nil {
		return nil, err
	}
	return &CellarV22CallOnAdaptorIterator{contract: _CellarV22.contract, event: "CallOnAdaptor", logs: logs, sub: sub}, nil
}

// WatchCallOnAdaptor is a free log subscription operation binding the contract event 0x261a6cb604cad99de116029b1ff284d989beb85ed58cd0f9042dd57363c04303.
//
// Solidity: event CallOnAdaptor((address,bytes[])[] data)
func (_CellarV22 *CellarV22Filterer) WatchCallOnAdaptor(opts *bind.WatchOpts, sink chan<- *CellarV22CallOnAdaptor) (event.Subscription, error) {

	logs, sub, err := _CellarV22.contract.WatchLogs(opts, "CallOnAdaptor")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarV22CallOnAdaptor)
				if err := _CellarV22.contract.UnpackLog(event, "CallOnAdaptor", log); err != nil {
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
func (_CellarV22 *CellarV22Filterer) ParseCallOnAdaptor(log types.Log) (*CellarV22CallOnAdaptor, error) {
	event := new(CellarV22CallOnAdaptor)
	if err := _CellarV22.contract.UnpackLog(event, "CallOnAdaptor", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarV22MulticallIterator is returned from FilterMulticall and is used to iterate over the raw logs and unpacked data for Multicall events raised by the CellarV22 contract.
type CellarV22MulticallIterator struct {
	Event *CellarV22Multicall // Event containing the contract specifics and raw log

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
func (it *CellarV22MulticallIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarV22Multicall)
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
		it.Event = new(CellarV22Multicall)
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
func (it *CellarV22MulticallIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarV22MulticallIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarV22Multicall represents a Multicall event raised by the CellarV22 contract.
type CellarV22Multicall struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterMulticall is a free log retrieval operation binding the contract event 0x9cad7b180826c2948ee4d20c7a113411aea819f34ec0fdc3342bbdafbd748f86.
//
// Solidity: event Multicall()
func (_CellarV22 *CellarV22Filterer) FilterMulticall(opts *bind.FilterOpts) (*CellarV22MulticallIterator, error) {

	logs, sub, err := _CellarV22.contract.FilterLogs(opts, "Multicall")
	if err != nil {
		return nil, err
	}
	return &CellarV22MulticallIterator{contract: _CellarV22.contract, event: "Multicall", logs: logs, sub: sub}, nil
}

// WatchMulticall is a free log subscription operation binding the contract event 0x9cad7b180826c2948ee4d20c7a113411aea819f34ec0fdc3342bbdafbd748f86.
//
// Solidity: event Multicall()
func (_CellarV22 *CellarV22Filterer) WatchMulticall(opts *bind.WatchOpts, sink chan<- *CellarV22Multicall) (event.Subscription, error) {

	logs, sub, err := _CellarV22.contract.WatchLogs(opts, "Multicall")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarV22Multicall)
				if err := _CellarV22.contract.UnpackLog(event, "Multicall", log); err != nil {
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

// ParseMulticall is a log parse operation binding the contract event 0x9cad7b180826c2948ee4d20c7a113411aea819f34ec0fdc3342bbdafbd748f86.
//
// Solidity: event Multicall()
func (_CellarV22 *CellarV22Filterer) ParseMulticall(log types.Log) (*CellarV22Multicall, error) {
	event := new(CellarV22Multicall)
	if err := _CellarV22.contract.UnpackLog(event, "Multicall", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// CellarV22OwnerUpdatedIterator is returned from FilterOwnerUpdated and is used to iterate over the raw logs and unpacked data for OwnerUpdated events raised by the CellarV22 contract.
type CellarV22OwnerUpdatedIterator struct {
	Event *CellarV22OwnerUpdated // Event containing the contract specifics and raw log

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
func (it *CellarV22OwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CellarV22OwnerUpdated)
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
		it.Event = new(CellarV22OwnerUpdated)
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
func (it *CellarV22OwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CellarV22OwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CellarV22OwnerUpdated represents a OwnerUpdated event raised by the CellarV22 contract.
type CellarV22OwnerUpdated struct {
	User     common.Address
	NewOwner common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOwnerUpdated is a free log retrieval operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_CellarV22 *CellarV22Filterer) FilterOwnerUpdated(opts *bind.FilterOpts, user []common.Address, newOwner []common.Address) (*CellarV22OwnerUpdatedIterator, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _CellarV22.contract.FilterLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &CellarV22OwnerUpdatedIterator{contract: _CellarV22.contract, event: "OwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchOwnerUpdated is a free log subscription operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_CellarV22 *CellarV22Filterer) WatchOwnerUpdated(opts *bind.WatchOpts, sink chan<- *CellarV22OwnerUpdated, user []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _CellarV22.contract.WatchLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CellarV22OwnerUpdated)
				if err := _CellarV22.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
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
func (_CellarV22 *CellarV22Filterer) ParseOwnerUpdated(log types.Log) (*CellarV22OwnerUpdated, error) {
	event := new(CellarV22OwnerUpdated)
	if err := _CellarV22.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
